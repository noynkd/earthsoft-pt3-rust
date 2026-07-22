// TSパケットの固定サイズ
pub const TS_PACKET_SIZE: usize = 188;

/// C++ のエラーコード（EC_VALIDなど）に相当する列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsResult {
    Valid,       // EC_VALID
    FormatError, // EC_FORMAT
    TxError,     // EC_TRANSPORT
    DropError,   // EC_CONTINUITY
}

/// TSパケットヘッダの構造体
#[derive(Debug, Default, Clone, Copy)]
pub struct TsHeader {
    pub sync_byte: u8,
    pub transport_error_indicator: bool,
    pub payload_unit_start_indicator: bool,
    pub transport_priority: bool,
    pub pid: u16,
    pub transport_scrambling_ctrl: u8,
    pub adaptation_field_ctrl: u8,
    pub continuity_counter: u8,
}

/// アダプテーションフィールドの構造体
#[derive(Debug, Default, Clone, Copy)]
pub struct AdaptationField {
    pub length: u8,
    pub discontinuity_indicator: bool,
    pub random_access_indicator: bool,
    pub es_priority_indicator: bool,
    pub pcr_flag: bool,
    pub opcr_flag: bool,
    pub splicing_point_flag: bool,
    pub transport_private_data_flag: bool,
    pub adaptation_field_ext_flag: bool,
    // オプションデータの位置（パケット内のインデックス範囲）
    pub option_range: Option<(usize, usize)>,
}

/// 188バイトのTSパケット全体を管理する構造体
#[derive(Debug, Clone)]
pub struct TsPacket {
    pub raw_data: [u8; TS_PACKET_SIZE],
    pub header: TsHeader,
    pub adaptation_field: AdaptationField,
}

impl TsPacket {
    /// 新しい空のパケットを生成 (C++のコンストラクタ相当)
    pub fn new(data: [u8; TS_PACKET_SIZE]) -> Self {
        Self {
            raw_data: data,
            header: TsHeader::default(),
            adaptation_field: AdaptationField::default(),
        }
    }

    /// C++ の ParsePacket を忠実に再現した解析・検証関数
    /// `continuity_counters`: PIDごとの前回のカウンタ値を保持する配列（通常 u8 が 8192 個の配列）
    pub fn parse_packet(&mut self, continuity_counters: &mut [u8; 8192]) -> TsResult {
        let data = &self.raw_data;

        // 1. TSパケットヘッダ解析 (ビットマスクとシフトの完全移植)
        self.header.sync_byte = data[0];
        self.header.transport_error_indicator = (data[1] & 0x80) != 0;
        self.header.payload_unit_start_indicator = (data[1] & 0x40) != 0;
        self.header.transport_priority = (data[1] & 0x20) != 0;
        self.header.pid = (((data[1] & 0x1F) as u16) << 8) | (data[2] as u16);
        self.header.transport_scrambling_ctrl = (data[3] & 0xC0) >> 6;
        self.header.adaptation_field_ctrl = (data[3] & 0x30) >> 4;
        self.header.continuity_counter = data[3] & 0x0F;

        // 2. アダプテーションフィールド解析
        self.adaptation_field = AdaptationField::default(); // ゼロ初期化に相当

        if (self.header.adaptation_field_ctrl & 0x02) != 0 {
            self.adaptation_field.length = data[4];
            if self.adaptation_field.length > 0 {
                self.adaptation_field.discontinuity_indicator = (data[5] & 0x80) != 0;
                self.adaptation_field.random_access_indicator = (data[5] & 0x40) != 0;
                self.adaptation_field.es_priority_indicator = (data[5] & 0x20) != 0;
                self.adaptation_field.pcr_flag = (data[5] & 0x10) != 0;
                self.adaptation_field.opcr_flag = (data[5] & 0x08) != 0;
                self.adaptation_field.splicing_point_flag = (data[5] & 0x04) != 0;
                self.adaptation_field.transport_private_data_flag = (data[5] & 0x02) != 0;
                self.adaptation_field.adaptation_field_ext_flag = (data[5] & 0x01) != 0;

                if self.adaptation_field.length > 1 {
                    // オプションデータのスライス範囲を記憶 (C++ のポインタ保持に相当)
                    let start = 6;
                    let end = 5 + self.adaptation_field.length as usize;
                    if end <= TS_PACKET_SIZE {
                        self.adaptation_field.option_range = Some((start, end));
                    }
                }
            }
        }

        // 3. パケットのフォーマット適合性チェック (C++ のガード節をそのまま移植)
        if self.header.sync_byte != 0x47 { return TsResult::FormatError; }
        if self.header.transport_error_indicator { return TsResult::TxError; }
        if self.header.pid >= 0x0002 && self.header.pid <= 0x000F { return TsResult::FormatError; }
        if self.header.transport_scrambling_ctrl == 0x01 { return TsResult::FormatError; }
        if self.header.adaptation_field_ctrl == 0x00 { return TsResult::FormatError; }
        
        let af_len = self.adaptation_field.length as usize;
        if self.header.adaptation_field_ctrl == 0x02 && af_len > 183 { return TsResult::FormatError; }
        if self.header.adaptation_field_ctrl == 0x03 && af_len > 182 { return TsResult::FormatError; }

        // 空パケット（Null Packet: PID 0x1FFF）の場合は連続性チェックをスキップ
        if self.header.pid == 0x1FFF {
            return TsResult::Valid;
        }

        // 4. 連続性チェック（ドロップ検知カウンタ判定）
        let pid_idx = self.header.pid as usize;
        let old_counter = continuity_counters[pid_idx];
        
        // ペイロードがあれば最新のカウンタ値を、なければ0x10（未定義/対象外値）を格納
        let has_payload = (self.header.adaptation_field_ctrl & 0x01) != 0;
        let new_counter = if has_payload { self.header.continuity_counter } else { 0x10 };
        continuity_counters[pid_idx] = new_counter;

        // 不連続フラグが立っていない場合のみドロップチェックを行う
        if !self.adaptation_field.discontinuity_indicator {
            if old_counter < 0x10 && new_counter < 0x10 {
                // カウンタは 0～15 (4ビット) で循環するため、+1 して 0x0F でマスク
                if ((old_counter.wrapping_add(1)) & 0x0F) != new_counter {
                    return TsResult::DropError;
                }
            }
        }

        TsResult::Valid
    }

    /// C++ の GetPayloadData / GetPayloadSize に相当する関数
    /// ペイロード部分を Rust の安全なスライス（参照）として切り出す
    pub fn get_payload(&self) -> Option<&[u8]> {
        match self.header.adaptation_field_ctrl {
            1 => Some(&self.raw_data[4..TS_PACKET_SIZE]), // ペイロードのみ
            3 => {
                let start = self.adaptation_field.length as usize + 5;
                if start < TS_PACKET_SIZE {
                    Some(&self.raw_data[start..TS_PACKET_SIZE]) // AF + ペイロードあり
                } else {
                    None
                }
            }
            _ => None, // アダプテーションフィールドのみ、または不正値
        }
    }
}
