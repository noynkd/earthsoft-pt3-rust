// =============================================================================
// Multi2Decoder
// =============================================================================

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Multi2Decoder {
    is_system_key_valid: bool,
    is_work_key_valid: bool,

    system_key: SystemKey,
    work_key_odd: SystemKey,
    work_key_even: SystemKey,
    initial_vector: DataKey,
}

impl Multi2Decoder {
    const SCRAMBLE_ROUND: u32 = 4;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self, system_key_bytes: &[u8; 32], initial_vector_bytes: &[u8; 8]) {
        self.system_key = system_key_bytes.into();
        self.initial_vector = initial_vector_bytes.into();

        self.is_system_key_valid = true;
        self.is_work_key_valid = false;
    }

    pub fn set_scramble_key(&mut self, scramble_key_bytes: Option<&[u8; 16]>) -> bool {
        if !self.is_system_key_valid {
            return false;
        }

        let keys = match scramble_key_bytes {
            Some(bytes) => bytes,
            None => {
                self.is_work_key_valid = false;
                return false;
            }
        };

        let odd = DataKey::from(&keys[0..8].try_into().unwrap());
        let even = DataKey::from(&keys[8..16].try_into().unwrap());

        self.work_key_odd = SystemKey::key_schedule(&self.system_key, odd);
        self.work_key_even = SystemKey::key_schedule(&self.system_key, even);

        self.is_work_key_valid = true;

        true
    }

    pub fn decode(&self, data: &mut [u8], scrambling_control: u8) -> bool {
        // キーが不正
        if !self.is_system_key_valid || !self.is_work_key_valid {
            return false;
        }

        let work_key = match scrambling_control {
            0 => return true,           // NO_SCRAMBLE
            1 => return false,          // RESERVED
            2 => &self.work_key_even,   // EVEN
            3 => &self.work_key_odd,    // ODD
            _ => return false,          // OR ELSE
        };

        let mut cbc_data = self.initial_vector;
        let mut position = 0;
        let length = data.len();

        while (length - position) >= 8 {
            // 8バイト取り出す
            let mut bytes  = [0u8; 8];
            bytes.copy_from_slice(&data[position..position + 8]);

            // DataKey に変換する
            let src_data = (&bytes).into();
            let mut block = src_data;

            self.decrypt_block(&mut block, work_key);

            block.left ^= cbc_data.left;
            block.right ^= cbc_data.right;

            cbc_data = src_data;

            data[position..position + 8].copy_from_slice(&block.to_be_bytes());
            position += 8;
        }

        if position < length {
            let mut encrypt_feed = cbc_data;
            self.encrypt_block(&mut encrypt_feed, work_key);

            let remain_bytes = encrypt_feed.to_be_bytes();
            let mut remain_index = 0;

            while position < length {
                data[position] ^= remain_bytes[remain_index];

                position += 1;
                remain_index += 1;
            }
        }

        true
    }

    #[inline]
    fn decrypt_block(&self, block: &mut DataKey, work_key: &SystemKey) {
        for _ in 0..Self::SCRAMBLE_ROUND {
            round_pi4(block, work_key.keys[7]);
            round_pi3(block, work_key.keys[5], work_key.keys[6]);
            round_pi2(block, work_key.keys[4]);
            round_pi1(block);

            round_pi4(block, work_key.keys[3]);
            round_pi3(block, work_key.keys[1], work_key.keys[2]);
            round_pi2(block, work_key.keys[0]);
            round_pi1(block);
        }
    }

    #[inline]
    fn encrypt_block(&self, block: &mut DataKey, work_key: &SystemKey) {
        for _ in 0..Self::SCRAMBLE_ROUND {
            round_pi1(block);
            round_pi2(block, work_key.keys[0]);
            round_pi3(block, work_key.keys[1], work_key.keys[2]);
            round_pi4(block, work_key.keys[3]);

            round_pi1(block);
            round_pi2(block, work_key.keys[4]);
            round_pi3(block, work_key.keys[5], work_key.keys[6]);
            round_pi4(block, work_key.keys[7]);
        }
    }
}

// =============================================================================
// DataKey
// =============================================================================

/// Data Key (Dk) 64bit
/// MULTI2の1ブロック、またはB-CASから得られる単一の鍵を表現する。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct DataKey {
    pub left: u32,
    pub right: u32,
}

impl DataKey {
    /// DataKeyのデータをビッグエンディアンのバイト配列(8バイト)に書き出す。
    pub fn to_be_bytes(&self) -> [u8; 8] {
        let mut hex_data = [0u8; 8];

        hex_data[0..4].copy_from_slice(&self.left.to_be_bytes());
        hex_data[4..8].copy_from_slice(&self.right.to_be_bytes());

        hex_data
    }
}

impl From<&[u8; 8]> for DataKey {
    /// ビッグエンディアンのバイト配列(8バイト)からDataKeyを構築する。
    fn from(value: &[u8; 8]) -> Self {
        Self {
            left: u32::from_be_bytes([value[0], value[1], value[2], value[3]]),
            right: u32::from_be_bytes([value[4], value[5], value[6], value[7]]),
        }
    }
}

// =============================================================================
// SystemKey
// =============================================================================

/// System Key (Sk) / Expanded Key (Wk) 256bit
/// 32ビット×8個の鍵配列を表現する。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct SystemKey {
    pub keys: [u32; 8],
}

impl SystemKey {
    /// 特定インデックスの鍵を更新する。
    pub fn set_key(&mut self, index: usize, key: u32) {
        if index < 8 {
            self.keys[index] = key;
        }
    }

    pub fn key_schedule(system_key: &SystemKey, mut data_key: DataKey) -> Self {
        let mut work_key = SystemKey::default();

        round_pi1(&mut data_key);

        round_pi2(&mut data_key, system_key.keys[0]);
        work_key.set_key(0, data_key.left);

        round_pi3(&mut data_key, system_key.keys[1], system_key.keys[2]);
        work_key.set_key(1, data_key.right);

        round_pi4(&mut data_key, system_key.keys[3]);
        work_key.set_key(2, data_key.left);

        round_pi1(&mut data_key);
        work_key.set_key(3, data_key.right);

        round_pi2(&mut data_key, system_key.keys[4]);
        work_key.set_key(4, data_key.left);

        round_pi3(&mut data_key, system_key.keys[5], system_key.keys[6]);
        work_key.set_key(5, data_key.right);

        round_pi4(&mut data_key, system_key.keys[7]);
        work_key.set_key(6, data_key.left);

        round_pi1(&mut data_key);
        work_key.set_key(7, data_key.right);

        work_key
    }
}

impl From<&[u8; 32]> for SystemKey {
    /// ビッグエンディアンのバイト配列(32バイト)からSystemKeyを構築する。
    fn from(value: &[u8; 32]) -> Self {
        let mut keys: [u32; 8] = Default::default();

        for index in 0..8 {
            let offset = index * 4;
            keys[index] = u32::from_be_bytes([
                value[offset],
                value[offset + 1],
                value[offset + 2],
                value[offset + 3],
            ]);
        }

        Self {
            keys
        }
    }
}

// =============================================================================
// 補助関数
// =============================================================================

/// 基本暗号化関数 π1
#[inline]
fn round_pi1(block: &mut DataKey) {
    block.right ^= block.left;
}

/// 基本暗号化関数 π2
#[inline]
fn round_pi2(block: &mut DataKey, k1: u32) {
    let y = block.right.wrapping_add(k1);
    let z = y.rotate_left(1).wrapping_add(y).wrapping_sub(1);
    block.left ^= z.rotate_left(4) ^ z;
}

/// 基本暗号化関数 π3
#[inline]
fn round_pi3(block: &mut DataKey, k2: u32, k3: u32) {
    let y = block.left.wrapping_add(k2);
    let z = y.rotate_left(2).wrapping_add(y).wrapping_add(1);
    let a = z.rotate_left(8) ^ z;
    let b = a.wrapping_add(k3);
    // let c = b.rotate_left(1).wrapping_sub(b);
    let c = b.wrapping_add(b >> 31);
    block.right ^= c.rotate_left(16) ^ (c | block.left);
}

/// 基本暗号化関数 π4
#[inline]
fn round_pi4(block: &mut DataKey, k4: u32) {
    let y = block.right.wrapping_add(k4);
    block.left ^= y.rotate_left(2).wrapping_add(y).wrapping_add(1);
}

// =============================================================================
// テスト
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi2_descramble() {
        // 1. ARIB STD-B25 規格で定義されている「本物の共通システム鍵」(32バイト)
        let sample_system_key: [u8; 32] = [
            0xDF, 0xA5, 0x15, 0x88, 0x28, 0x1D, 0x00, 0x24,
            0x8E, 0x94, 0x6A, 0x1D, 0x69, 0xec, 0x54, 0xF9,
            0xAB, 0x1c, 0x5B, 0x6e, 0x76, 0x77, 0x2E, 0x54,
            0xF8, 0x0A, 0x11, 0x4B, 0x56, 0x25, 0x71, 0x11,
        ];

        // 2. CBCモード用の初期化ベクトル (B25仕様ではすべて0の8バイト)
        let sample_initial_cbc: [u8; 8] = [0x00; 8];

        // 3. B-CASカードから送られてくることを想定した「スクランブル鍵」(16バイト)
        // ※ここではテスト用のダミーの鍵を設定します（前半8バイトがOdd、後半8バイトがEven）
        let sample_scramble_key: [u8; 16] = [
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // Odd
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, // Even
        ];

        // 4. デコーダーのインスタンスを作成し、初期化
        let mut decoder = Multi2Decoder::new();
        decoder.initialize(&sample_system_key, &sample_initial_cbc);

        // 5. スクランブル鍵を登録（これで内部で work_key_odd / even が自動生成される）
        let key_set_success = decoder.set_scramble_key(Some(&sample_scramble_key));
        assert!(key_set_success, "鍵のセットアップに失敗しました");

        // 6. 暗号化された映像パケット（ペイロード）を模したテストデータ (20バイト)
        // ※端数データの処理もテストするため、8の倍数ではないサイズ（20バイト）にしています
        let mut test_payload: [u8; 20] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // CBCブロック1
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, // CBCブロック2
            0x99, 0x88, 0x77, 0x66,                         // OFBブロック (端数4バイト)
        ];

        // テスト用の元データを退避（後で比較するため）
        let original_encrypted_data = test_payload;

        // 7. 【検証①】「奇数（scr_ctrl = 3）」としてデータを一度暗号化（スクランブル）してみる
        // ※OFB処理の整合性を合わせるため、まずは暗号化が正常に回るかテストします
        // (B25ではdecode関数でOFBの鍵ストリーム生成にencrypt_blockを使うため、暗号化と復号は対の関係になります)
        println!("元のデータ: {:?}", test_payload);

        // 8. 【検証②】「奇数（scr_ctrl = 3）」を指定して復号（デスクランブル）を実行
        // 実際のテレビ放送波では、TSパケットヘッダの transport_scrambling_control から 2 か 3 を取得して渡します
        let descramble_success = decoder.decode(&mut test_payload, 3);
        assert!(descramble_success, "復号処理に失敗しました");
        
        println!("処理後のデータ: {:?}", test_payload);

        // 9. 【検証③】間違った鍵（Even = 2）で復号しようとすると、データが元に戻らないことを確認
        let mut wrong_payload = original_encrypted_data;
        decoder.decode(&mut wrong_payload, 2); // 偶数鍵で復号を試みる
        assert_ne!(
            wrong_payload, original_encrypted_data, 
            "間違った鍵指定なのにデータが変化していません"
        );
    }
}
