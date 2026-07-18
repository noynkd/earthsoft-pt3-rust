use crate::buffer;
use crate::utility;
use earthsoft_sdk::pt3;

pub fn set_lnb_power(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    let power = device.get_lnb_power()?;

    let labels = vec!["オフ", "+15V", "+11V"];

    println!("[LNB 電源設定]");
    println!("0: (戻る)");
    for (index, lnb_power) in pt3::LnbPower::ALL.iter().enumerate() {
        println!("{}: {} {}",
            index + 1,
            labels[index],
            if power.eq(lnb_power) { "[設定値]" } else { "" }
        );
    }

    let power = match utility::get_number(pt3::LnbPower::COUNT as u32)
    {
        0 => return Ok(()),
        number => pt3::LnbPower::try_from(number - 1)
            .inspect_err(|e| {
                eprintln!("pt3::LnbPower::try_from() に失敗しました: {:?}", e);
            })?,
    };

    device.set_lnb_power(power)
}

pub fn scan_channel(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    let (isdb, tuner) = match select_tuner(false) {
        Ok(value) => value,
        Err(pt3::Error::NotImplemented) => return Ok(()),
        Err(other) => return Err(other),
    };

    match isdb {
        pt3::Isdb::Satellite => scan_satellite_channels(&device, tuner, 0..=23)
            .inspect_err(|e| {
                eprintln!("command::scan_satellite_channels() に失敗しました: {:?}", e);
            })?,
        pt3::Isdb::Terrestrial => scan_terrestrial_channels(&device, tuner, 0..=112)
            .inspect_err(|e| {
                eprintln!("command::scan_terrestrial_channels() に失敗しました: {:?}", e);
            })?,
    };

    Ok(())
}

pub fn set_channel(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    let (isdb, tuner) = match select_tuner(false) {
        Ok(value) => value,
        Err(pt3::Error::NotImplemented) => return Ok(()),
        Err(other) => return Err(other),
    };

    let max_channel = match isdb {
        pt3::Isdb::Satellite => 23,
        pt3::Isdb::Terrestrial => 112,
    };

    println!("チャンネル番号を入力してください。(範囲:0～{})", max_channel);

    let channel = utility::get_number(max_channel);

    match isdb {
        pt3::Isdb::Satellite => scan_satellite_channels(&device, tuner, channel..=channel)
            .inspect_err(|e| {
                eprintln!("command::scan_satellite_channels() に失敗しました: {:?}", e);
            })?,
        pt3::Isdb::Terrestrial => scan_terrestrial_channels(&device, tuner, channel..=channel)
            .inspect_err(|e| {
                eprintln!("command::scan_terrestrial_channels() に失敗しました: {:?}", e);
            })?,
    };

    Ok(())
}

pub fn set_ts_id(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    let (_, tuner) = match select_tuner(true) {
        Ok(value) => value,
        Err(pt3::Error::NotImplemented) => return Ok(()),
        Err(other) => return Err(other),
    };

    println!("TS-ID を入力してください。(範囲:0x0000～0xFFFF");

    let id = utility::get_hex_number(0xFFFF);

    device.set_satellite_id(tuner, id)
        .inspect_err(|e| {
            eprintln!("pt3::Device::set_satellite_id() に失敗しました: {:?}", e);
        })?;

    Ok(())
}

pub fn show_error_rate_count(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    println!("--+-----+---+--------+----------------+--------+--------+--------+--------");
    println!("   推定  AGC RFレベル エラーパケット数 [誤り訂正されたビットレート       ]");
    println!("   C/N          (dBm)                  [リードソロモン          ] ビタビ");
    println!("    (dB)                               低階層   高階層");
    println!("                                        A階層    B階層    C階層");
    println!("--+-----+---+--------+----------------+--------+--------+--------+--------");

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            print!("{}{}", if isdb == pt3::Isdb::Satellite { "S" } else { "T" }, tuner);

            let (cn100, current_agc, _max_agc) = device.get_cn_agc(isdb, tuner)
                .inspect_err(|e| {
                    eprintln!("pt3::Device::get_cn_agc() に失敗しました: {:?}", e);
                })?;
            print!(" {:5.2} {:3}", cn100 as f64 / 100.0, current_agc);

            if isdb == pt3::Isdb::Terrestrial {
                let rf_level = device.get_rf_level(tuner)
                    .inspect_err(|e| {
                        eprintln!("pt3::Device::get_rf_level() に失敗しました: {:?}", e);
                    })?;
                print!(" {:8.3}", rf_level);
            } else {
                print!("         ");
            };

            let error_count = device.get_error_count(isdb, tuner)
                .inspect_err(|e| {
                    eprintln!("pt3::Device::get_error_count() に失敗しました: {:?}", e);
                })?;
            print!(" {:16}", error_count & 0x00ff_ffff);

            let layer_count = if isdb == pt3::Isdb::Satellite {
                pt3::SatelliteLayerIndex::COUNT as u32
            } else {
                pt3::TerrestrialLayerIndex::COUNT as u32
            };

            for layer in 0..layer_count {
                let error_rate = device.get_corrected_error_rate(isdb, tuner, layer)
                    .inspect_err(|e| {
                        eprintln!("pt3::Device::get_corrected_error_rate() に失敗しました: {:?}", e);
                    })?;
                if error_rate.numerator == 0 || error_rate.denominator == 0 {
                    print!("{:8}", 0);
                } else {
                    print!("{:8.2e}", error_rate.numerator as f64 / error_rate.denominator as f64);
                }
            }

            if isdb == pt3::Isdb::Satellite {
                print!("         ");
            }

            let error_rate = device.get_inner_error_rate(isdb, tuner)
                .inspect_err(|e| {
                    eprintln!("pt3::Device::get_inner_error_rate() に失敗しました: {:?}", e);
                })?;
            if error_rate.numerator == 0 || error_rate.denominator == 0 {
                print!("{:8}", 0);
            } else {
                print!("{:8.2e}", error_rate.numerator as f64 / error_rate.denominator as f64);
            }

            println!();
        }
    }

    Ok(())
}

pub fn check_hardware(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    if !check_dma_transfer_enabled(&device)? {
        return Ok(());
    }

    let result = (|| {
        println!("- 固定部が正しいかチェックしています...");
        if !check_constant_info(&device) {
            return false;
        }

        println!("- TS ピンをチェックしています...");
        if !check_ts_pins(&device) {
            return false;
        }

        println!("- TS 同期バイトをチェックしています...");
        if !check_ts_sync_byte(&device) {
            return false;
        }

        println!("- チューナーの PLL がロックするかチェックしています...");
        if !check_tuner_pll(&device) {
            return false;
        }

        println!("- テストデータを転送してデータをチェックしています...");
        if !check_transfer(&device, false) || !check_transfer(&device, true) {
            cleanup_transfer(&device);
            return false;
        }
        cleanup_transfer(&device);

        true
    })();

    if let Err(e) = device.set_ram_pins_mode(pt3::RamPinsMode::Normal) {
        eprintln!("pt3::Device::set_ram_pins_mode() に失敗しました: {:?}", e);
    }

    if result {
        println!("┌─────────────┐");
        println!("│ＯＫ  正常に完了しました。│");
        println!("└─────────────┘");
    } else {
        println!("■■■■■■■■■■■■■■■■■");
        println!("■ ＮＧ  エラーが発生しました。 ■");
        println!("■■■■■■■■■■■■■■■■■");
    }

    Ok(())
}

pub fn set_amp_power(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    println!("[AMP 電源設定]");
    println!("0: (戻る)");
    println!("1: オフ");
    println!("2: オン");

    let power = match utility::get_number(2) {
        0 => return Ok(()),
        number => number == 2,
    };

    device.set_amp_power(power)
}

pub fn set_tuner_sleep(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    let (isdb, tuner) = match select_tuner(false) {
        Ok(value) => value,
        Err(pt3::Error::NotImplemented) => return Ok(()),
        Err(other) => return Err(other),
    };

    let sleep = device.get_tuner_sleep(isdb, tuner)?;

    let labels = vec!["無効", "有効"];

    println!("[チューナースリープ設定]");
    println!("0: (戻る)");

    for (index, label) in labels.iter().enumerate() {
        println!("{}: {} {}",
            index + 1,
            label,
            if index == sleep as usize { "[設定値]" } else { "" }
        );
    }

    let sleep = match utility::get_number(labels.len() as u32) {
        0 => return Ok(()),
        selected => selected == 2,
    };

    device.set_tuner_sleep(isdb, tuner, sleep)
}

pub fn scan_test(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    _ = scan_satellite_channels(&device, 0, 7..=7)?;
    _ = scan_satellite_channels(&device, 1, 15..=15)?;
    _ = scan_terrestrial_channels(&device, 0, 70..=70)?;
    _ = scan_terrestrial_channels(&device, 1, 71..=71)?;

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            device.reset_corrected_error_count(isdb, tuner)?;
        }
    }

    Ok(())
}

pub fn check_eratta(device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
    if !check_dma_transfer_enabled(&device)? {
        return Ok(());
    }

    println!("試行回数を入力してください。(範囲:0～9)");

    let test_count = utility::get_number(9) as usize;
    if test_count == 0 {
        return Ok(());
    }

    for index in 0..test_count {
        println!("{} 回目の試行を開始します。", index);

        if !check_eratta_transfer(&device) {
            return Ok(());
        }

        println!("{} 回目の試行は正常に終了しました。", index);
    }

    cleanup_transfer(&device);

    println!("すべての試行は正常に終了しました。");

    Ok(())
}

fn scan_satellite_channels(device: &std::sync::Arc<pt3::Device>, tuner: u32, channels: impl IntoIterator<Item = u32>) -> Result<(), pt3::Error> {
    println!("                        変:変更指示 / 起:起動制御信号 / ア:アップリンク制御情報");
    println!("---+----+---+-------+------+--+--+--+-------------------+----------------------");
    println!("No. Ch.  AGC Δclock Δcarr 変 起 ア 伝送モード/Slot数   TS-ID (Hex)");
    println!("        /128   (ppm)  (kHz)          1    2    3    4    1    2    3    4    5");
    println!("---+----+---+-------+------+--+--+--+-------------------+----------------------");

    for channel in channels {
        scan_satellite_channel(&device, tuner, channel)?;
    }

    println!("---+----+---+-------+------+--+--+--+-------------------+----------------------");

    Ok(())
}

fn scan_satellite_channel(device: &std::sync::Arc<pt3::Device>, tuner: u32, channel: u32) -> Result<(), pt3::Error> {
    let (is_bs, number) = get_satellite_channel_name(channel);
    print!("{:3} {}{:02}"
        , channel
        , if is_bs { "BS" } else { "ND" }
        , number);

    device.set_frequency(pt3::Isdb::Satellite, tuner, channel, 0)?;

    let start_time = std::time::Instant::now();

    let tmcc_option = loop {
        match device.get_satellite_tmcc(tuner) {
            Ok(tmcc) => {
                break Some(tmcc);
            },
            Err(pt3::Error::Unspecified) => {
                // HACK: TMCC 受信待ち
                // eprintln!("pt3::Device::get_satellite_tmcc() に失敗しました: {}", e);
                if start_time.elapsed() > std::time::Duration::from_secs(2) {
                    break None; // タイムアウト
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            },
            Err(e) => {
                return Err(e);
            },
        };
    };

    let (_cn100, current_agc, _max_agc) = device.get_cn_agc(pt3::Isdb::Satellite, tuner)?;

    print!(" {:3}", current_agc);

    let Some(tmcc) = tmcc_option else {
        println!("(TMCC 受信不可)");
        return Ok(());
    };

    let (clock, offset) = device.get_frequency_offset(pt3::Isdb::Satellite, tuner)?;

    print!("{:+7.2} {:+6}", clock as f64 / 100.0, offset / 1000 );
    print!(" {:2} {:2} {:2}", tmcc.indicator, tmcc.emergency, tmcc.up_link);

    for index in 0..4 {
        let mode = tmcc.mode[index];
        let slot = tmcc.slot[index];

        if mode == 0x0f {
            print!(" -/--");
        } else {
            print!(" {}/{:02}", mode, slot);
        }
    }

    let mut last_index = 0;
    for index in 0..8 {
        if tmcc.id[index] != 0xFFFF {
            last_index = index;
        }
    }

    for index in 0..8 {
        if tmcc.id[index] != 0xFFFF {
            print!(" {:04x}", tmcc.id[index]);
        } else if index < last_index {
            print!(" ----");
        }
    }

    println!();

    Ok(())
}

fn get_satellite_channel_name(channel: u32) -> (bool, u32) {
    if channel < 12 {
        (true, 1 + 2 * channel)
    } else if channel < 24 {
        (false, 2 + 2 * (channel - 12))
    } else {
        (false, 1 + 2 * (channel - 24))
    }
}

fn scan_terrestrial_channels(device: &std::sync::Arc<pt3::Device>, tuner: u32, channels: impl IntoIterator<Item = u32>) -> Result<(), pt3::Error> {
    println!("---+---+---+-------+------+-+------------------------------------------");
    println!("No. Ch. AGC Δclock Δcarr S 変調/符号化率/インターリーブ/セグメント数");
    println!("        255   (ppm)  (kHz)   A階層    B階層    C階層");
    println!("---+---+---+-------+------+-+------------------------------------------");

    for channel in channels {
        scan_terrestrial_channel(&device,tuner, channel)?;
    }

    println!("---+---+---+-------+------+-+------------------------------------------");

    Ok(())
}

fn scan_terrestrial_channel(device: &std::sync::Arc<pt3::Device>, tuner: u32, channel: u32) -> Result<(), pt3::Error> {
    let (is_catv, number) = get_terrestrial_channel_name(channel);
    print!("{:3} {}{:02}"
        , channel
        , if is_catv { "C" } else { " " }
        , number);

    device.set_frequency(pt3::Isdb::Terrestrial, tuner, channel, 0)?;

    let tmcc_option = match device.get_terrestrial_tmcc(tuner) {
        Ok(tmcc) => Some(tmcc),
        Err(pt3::Error::Unspecified) => None,
        Err(e) => {
            return Err(e);
        },
    };

    let (_cn100, current_agc, _max_agc) = device.get_cn_agc(pt3::Isdb::Terrestrial, tuner)?;

    print!(" {:3}", current_agc);

    let Some(tmcc) = tmcc_option else {
        println!("(TMCC 受信不可)");
        return Ok(());
    };

    let (clock, offset) = device.get_frequency_offset(pt3::Isdb::Terrestrial, tuner)?;

    print!("{:+7.2} {:+6}", clock as f64 / 100.0, offset / 1000 );

    print!(" {}", tmcc.system);

    for index in 0..3 {
        let mode = tmcc.mode[index];
        let rate = tmcc.rate[index];
        let interleave = tmcc.interleave[index];
        let segment = tmcc.segment[index];

        if mode == 7 {
            print!(" -/-/-/");
        } else {
            print!(" {}/{}/{}/", mode, rate, interleave);
        }

        if segment == 15 {
            print!(" --");
        } else {
            print!(" {:02}", segment);
        }
    }

    println!();

    Ok(())
}

fn get_terrestrial_channel_name(channel: u32) -> (bool, u32) {
    const RANGES: [(u32, bool, u32); 5] = [
        (  2, false,  3),
        ( 12,  true, 22),
        ( 21, false, 12),
        ( 62,  true, 63),
        (112, false, 62),
    ];

    for &(max_channel, is_catv, offset) in &RANGES {
        if channel <= max_channel {
            return (is_catv, channel + offset - max_channel);
        }
    }

    (false, 0u32)
}

fn select_tuner(satellite_only: bool) -> Result<(pt3::Isdb, u32), pt3::Error> {
    println!("0: (戻る)");
    println!("1: S1");
    println!("2: S2");
    if !satellite_only {
        println!("3: T1");
        println!("4: T2");
    }

    match utility::get_number(if satellite_only { 2 } else { 4 }) {
        0 => Err(pt3::Error::NotImplemented),
        selected => {
            let isdb = pt3::Isdb::try_from((selected - 1) / 2)?;
            let tuner = (selected - 1) % 2;

            Ok((isdb, tuner))
        },
    }
}

fn check_dma_transfer_enabled(device: &std::sync::Arc<pt3::Device>) -> Result<bool, pt3::Error> {
    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let enabled = device.get_transfer_enabled(isdb, tuner)?;
            if enabled {
                println!("すべてのDMAが停止状態ではないため実行できません。");
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn check_constant_info(device: &std::sync::Arc<pt3::Device>) -> bool {
    let constant_info = match device.get_constant_info() {
        Ok(constant_info) => constant_info,
        Err(_) => {
            return false;
        },
    };

    if constant_info.pt_version != 0x03 {
        println!("  - pt_version ({:#04x}) が誤っています。", constant_info.pt_version);
        return false;
    }

    if constant_info.register_map_version != 0x01 {
        println!("  - register_map_version ({:#04x}) が誤っています。", constant_info.register_map_version);
        return false;
    }

    if constant_info.fpga_version != 0x04 {
        println!("  - fpga_version ({:#04x}) が誤っています。", constant_info.fpga_version);
        return false;
    }

    if !constant_info.is_ts_supported {
        println!("  - is_ts_supported ({}) が誤っています。", constant_info.is_ts_supported);
        return false;
    }

    true
}

fn check_ts_pins(device: &std::sync::Arc<pt3::Device>) -> bool {
    for ram_pins_mode in pt3::RamPinsMode::ALL.iter().rev().copied() {
        if let Err(_) = device.set_ram_pins_mode(ram_pins_mode) {
            return false;
        }

        for index in 0..256 {
            let mut level = 0;

            if !check_ts_pins_level(&device, index, &mut level) {
                return false;
            }

            if index != level {
                println!("{:02x}, {:02x}", index, level);
            }
        }

        println!("  - RAM ピンモード {:?} は問題ありません。", ram_pins_mode);
    }

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            if let Err(_) = device.set_ts_pins_mode(isdb, tuner, pt3::TsPinsMode::default()) {
                // return false;
            }

        }
    }

    std::thread::sleep(std::time::Duration::from_millis(10));

    true
}

fn check_ts_pins_level(device: &std::sync::Arc<pt3::Device>, mode: u32, level: &mut u32) -> bool {
    let mut table = [[pt3::TsPinsMode::default(); 2]; 2];
    let mut bit = 0;

    for row in &mut table {
        for pins in row {
            pins.byte  = if (mode & (1 << (bit + 0))) != 0 { pt3::TsPinMode::High } else { pt3::TsPinMode::Low };
            pins.valid = if (mode & (1 << (bit + 1))) != 0 { pt3::TsPinMode::High } else { pt3::TsPinMode::Low };

            bit += 2;
        }
    }

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let pins = &mut table[isdb as usize][tuner as usize];
            let mut clocks = [false; 4];

            for clock in &mut clocks {
                for index in 0..2 {
                    pins.clock_data = if index == 0 { pt3::TsPinMode::High } else { pt3::TsPinMode::Low };

                    if let Err(_) = device.set_ts_pins_mode(isdb, tuner, *pins) {
                        // return false;
                    }

                    if index == 0 {
                        let ts_pins_level = device.get_ts_pins_level(isdb, tuner)
                            .unwrap_or_default();

                        *clock = ts_pins_level.clock;
                    }
                }
            }

            if clocks[0] == clocks[1] || clocks[1] == clocks[2] || clocks[2] == clocks[3] {
                println!("  - TS クロックに異常があります。");
                return false;
            }
        }
    }

    let mut culcurated_level = 0;
    let mut bit = 0;

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let ts_pins_level = device.get_ts_pins_level(isdb, tuner)
                .unwrap_or_default();

            culcurated_level |= (if ts_pins_level.byte  { 1 } else { 0 }) << (bit + 0);
            culcurated_level |= (if ts_pins_level.valid { 1 } else { 0 }) << (bit + 1);

            bit += 2;
        }
    }

    *level = culcurated_level;

    true
}

fn check_ts_sync_byte(device: &std::sync::Arc<pt3::Device>) -> bool {
    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let sync_byte = match device.get_ts_sync_byte(isdb, tuner) {
                Ok(sync_byte) => sync_byte,
                Err(_) => {
                    return false;
                },
            };

            if sync_byte != 0x47 {
                println!("  - 同期バイトに異常があります。 ({:#04x})", sync_byte);
                return false;
            }
        }
    }

    true
}

fn check_tuner_pll(device: &std::sync::Arc<pt3::Device>) -> bool {
    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            if let Err(_) = device.set_frequency(isdb, tuner, 0, 0) {
                return false;
            };
        }
    }

    true
}

fn cleanup_transfer(device: &std::sync::Arc<pt3::Device>) {
    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            if let Err(_) = device.set_transfer_enabled(isdb, tuner, false) {
                // return false;
            };
            if let Err(_) = device.set_transfer_test_mode(isdb, tuner, false, 0, false) {
                // return false;
            };
        }
    }
}

fn check_transfer(device: &std::sync::Arc<pt3::Device>, not_op_lfsr: bool) -> bool {
    let mut buffers: [[buffer::RingBuffer; 2]; 2] = Default::default();

    for isdb in pt3::Isdb::ALL {
        let device = std::sync::Arc::clone(&device);
        for tuner in 0..2 {
            let mut buffer = buffer::RingBuffer::new(
                1024 * 1024,
                1,
            );

            if let Err(e) = buffer.allocate(device.clone(), false) {
                eprintln!("buffer::RingBuffer::allocate() に失敗しました: {:?}", e);
                return false;
            }

            if let Err(e) = device.set_transfer_page_descriptor_address(isdb, tuner, buffer.get_descriptor_address()) {
                eprintln!("pt3::Device::set_transfer_page_descriptor_address() に失敗しました: {:?}", e);
                return false;
            }

            buffers[isdb as usize][tuner as usize] = buffer;
        }
    }

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let buffer = &mut buffers[isdb as usize][tuner as usize];

            buffer.slice_block_mut(0).fill(0);

            if let Err(e) = buffer.sync_cpu(0) {
                eprintln!("buffer::RingBuffer::sync_cpu() に失敗しました: {:?}", e);
                return false;
            }

            if let Err(e) = device.set_transfer_test_mode(
                isdb,
                tuner,
                true,
                get_transfer_lfsr(isdb, tuner),
                not_op_lfsr
            ) {
                eprintln!("pt3::Device::set_transfer_test_mode() に失敗しました: {:?}", e);
                return false;
            }

            if let Err(e) = device.set_transfer_enabled(isdb, tuner, true) {
                eprintln!("pt3::Device::set_transfer_enabled() に失敗しました: {:?}", e);
                return false;
            }
        }
    }

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let start_time = std::time::Instant::now();

            let transfer_info_option = loop {
                match device.get_transfer_info(isdb, tuner) {
                    Ok(transfer_info) => {
                        if !transfer_info.busy {
                            break Some(transfer_info);
                        }
                        if start_time.elapsed() > std::time::Duration::from_secs(2) {
                            break None; // タイムアウト
                        }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        continue;
                    },
                    Err(e) => {
                        eprintln!("pt3::Device::get_transfer_info() に失敗しました: {:?}", e);
                        return false;
                    },
                }
            };

            if transfer_info_option.is_none() {
                println!("  - 転送が完了しませんでした。");
                return false;
            };

            let buffer = &mut buffers[isdb as usize][tuner as usize];

            if let Err(e) = buffer.sync_io(0) {
                eprintln!("buffer::RingBuffer::sync_io() に失敗しました: {:?}", e);
                return false;
            }

            let bytes = buffer.slice_block(0);
            let words = unsafe {
                std::slice::from_raw_parts(
                    bytes.as_ptr() as *const u16,
                    bytes.len() / 2,
                )
            };
            let mut lfsr = get_transfer_lfsr(isdb, tuner);

            for current in words {
                if *current != if not_op_lfsr { lfsr ^ 0xffffu16 } else { lfsr } {
                    println!("  - 転送データに誤りがありました。current: {:#06x} lfsr: {:#06x} not: {:#06x} flag: {}", current, lfsr, lfsr ^ 0xffffu16, not_op_lfsr);
                    return false;
                }
                lfsr = (lfsr >> 1) ^ ((if (lfsr & 1) == 1 { 0xffffu16 } else { 0u16 }) & 0xb400u16);
            }
        }
    }

    true
}

fn check_eratta_transfer(device: &std::sync::Arc<pt3::Device>) -> bool {
    let mut buffers: [[buffer::RingBuffer; 2]; 2] = Default::default();

    for isdb in pt3::Isdb::ALL {
        let device = std::sync::Arc::clone(&device);
        for tuner in 0..2 {
            let mut buffer = buffer::RingBuffer::new(
                1024 * 1024,
                1,
            );

            if let Err(e) = buffer.allocate(device.clone(), false) {
                eprintln!("buffer::RingBuffer::allocate() に失敗しました: {:?}", e);
                return false;
            }

            if let Err(e) = device.set_transfer_page_descriptor_address(isdb, tuner, buffer.get_descriptor_address()) {
                eprintln!("pt3::Device::set_transfer_page_descriptor_address() に失敗しました: {:?}", e);
                return false;
            }

            buffers[isdb as usize][tuner as usize] = buffer;
        }
    }

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let buffer = &mut buffers[isdb as usize][tuner as usize];

            buffer.slice_block_mut(0).fill(0);

            if let Err(e) = buffer.sync_cpu(0) {
                eprintln!("buffer::RingBuffer::sync_cpu() に失敗しました: {:?}", e);
                return false;
            }

            if let Err(e) = device.set_transfer_test_mode(
                isdb,
                tuner,
                true,
                get_transfer_lfsr(isdb, tuner),
                false,
            ) {
                eprintln!("pt3::Device::set_transfer_test_mode() に失敗しました: {:?}", e);
                return false;
            }

            if let Err(e) = device.set_transfer_enabled(isdb, tuner, true) {
                eprintln!("pt3::Device::set_transfer_enabled() に失敗しました: {:?}", e);
                return false;
            }
        }
    }

    let stop_isdb = (rand::random_range(0..=1) as u32).try_into().unwrap_or_default();
    let stop_tuner = rand::random_range(0..=1);
    let stop_mills = rand::random_range(10..=49);

    std::thread::sleep(std::time::Duration::from_millis(stop_mills));

    if let Err(e) = device.set_transfer_enabled(stop_isdb, stop_tuner, false) {
        eprintln!("pt3::Device::set_transfer_enabled() に失敗しました: {:?}", e);
        return false;
    }

    if let Err(e) = device.set_transfer_test_mode(stop_isdb, stop_tuner, false, 0, false) {
        eprintln!("pt3::Device::set_transfer_test_mode() に失敗しました: {:?}", e);
        return false;
    }

    println!("- ISDB-{}{} を停止しました。", if stop_isdb == pt3::Isdb::Satellite { "S" } else { "T" }, stop_tuner);

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            let start_time = std::time::Instant::now();

            let transfer_info_option = loop {
                match device.get_transfer_info(isdb, tuner) {
                    Ok(transfer_info) => {
                        let mut done = false;

                        if transfer_info.internal_fifo_a_overflow {
                            println!("- Internal FIFO A Overflow が発生しました。");
                            done = true;
                        }
                        if transfer_info.internal_fifo_a_underflow {
                            println!("- Internal FIFO A Underflow が発生しました。");
                            done = true;
                        }
                        if transfer_info.external_fifo_overflow {
                            println!("- External FIFO Overflow が発生しました。");
                            done = true;
                        }
                        if transfer_info.internal_fifo_b_overflow {
                            println!("- Internal FIFO B Overflow が発生しました。");
                            done = true;
                        }
                        if transfer_info.internal_fifo_b_underflow {
                            println!("- Internal FIFO B Underflow が発生しました。");
                            done = true;
                        }

                        if !transfer_info.busy || done {
                            break Some(transfer_info);
                        }
                        if start_time.elapsed() > std::time::Duration::from_secs(2) {
                            break None; // タイムアウト
                        }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        continue;
                    },
                    Err(e) => {
                        eprintln!("pt3::Device::get_transfer_info() に失敗しました: {:?}", e);
                        return false;
                    },
                }
            };

            if transfer_info_option.is_none() {
                println!("  - 転送が完了しませんでした。");
                return false;
            };
        }
    }

    for isdb in pt3::Isdb::ALL {
        for tuner in 0..2 {
            println!("- ISDB-{}{} の転送データを検証します。", if isdb == pt3::Isdb::Satellite { "S" } else { "T" }, tuner);

            if isdb == stop_isdb && tuner == stop_tuner {
                println!("- ISDB-{}{} は停止しているためスキップします。", if isdb == pt3::Isdb::Satellite { "S" } else { "T" }, tuner);
                continue;
            }

            let buffer = &mut buffers[isdb as usize][tuner as usize];

            if let Err(e) = buffer.sync_io(0) {
                eprintln!("buffer::RingBuffer::sync_io() に失敗しました: {:?}", e);
                return false;
            }

            let bytes = buffer.slice_block(0);
            let words = unsafe {
                std::slice::from_raw_parts(
                    bytes.as_ptr() as *const u16,
                    bytes.len() / 2,
                )
            };
            let mut lfsr = get_transfer_lfsr(isdb, tuner);

            for current in words {
                if *current != lfsr {
                    println!("  - 転送データに誤りがありました。current: {:#06x} lfsr: {:#06x}", current, lfsr);
                    return false;
                }
                lfsr = (lfsr >> 1) ^ ((if (lfsr & 1) == 1 { 0xffffu16 } else { 0u16 }) & 0xb400u16);
            }

            println!("- ISDB-{}{} の転送データは問題ありませんでした。", if isdb == pt3::Isdb::Satellite { "S" } else { "T" }, tuner);
        }
    }

    cleanup_transfer(&device);

    true
}

fn get_transfer_lfsr(isdb: pt3::Isdb, tuner: u32) -> u16 {
    ((1 + 2 * isdb as u32 + tuner) * 12345) as u16
}
