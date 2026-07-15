mod buffer;
mod command;
mod context;
mod utility;

use earthsoft_sdk::pt3;

fn main() -> Result<(), pt3::Error> {
    println!("earthsoft-pt3-example version {}", env!("CARGO_PKG_VERSION"));

    let bus = context::BusContext::new()
        .inspect_err(|e| {
            eprintln!("context::BusContext::new() に失敗しました: {:?}", e);
        })?;

    show_bus_menu(bus.clone())
        .inspect_err(|e| {
            eprintln!("main::show_bus_menu() に失敗しました: {:?}", e);
        })?;

    Ok(())
}

fn show_bus_menu(bus: std::sync::Arc<context::BusContext>) -> Result<(), pt3::Error> {
    let device_infos = bus.scan_device_info(9)
        .inspect_err(|e| {
            eprintln!("context::BusContext::scan_device_info() に失敗しました: {:?}", e);
        })?;

    loop {
        println!("[デバイス選択メニュー]");
        println!("   Bus:バス番号 / Dev:デバイス番号 / Fun:ファンクション番号 / PTn:品番");
        println!("--+---+---+---+---");
        println!("#  Bus Dev Fun PTn");
        println!("--+---+---+---+---");

        println!("0: (終了)");

        for (index, device_info) in device_infos.iter().enumerate() {
            println!("{}: {:3} {:3} {:3} {:3}",
                index + 1,
                device_info.bus,
                device_info.slot,
                device_info.function,
                device_info.pt_version);
        }

        match utility::get_number(device_infos.len() as u32) {
            0 => return Ok(()),
            number => {
                let device_info = &device_infos[(number - 1) as usize];
                let device = bus.create_device(device_info)
                    .inspect_err(|e| {
                        eprintln!("context::BusContext::allocate() に失敗しました: {:?}", e);
                    })?;
                init_device(&device)
                    .inspect_err(|e| {
                        eprintln!("main::init_device() に失敗しました: {:?}", e);
                    })?;
                show_device_menu_page_1(device.clone())
                    .inspect_err(|e| {
                        eprintln!("main::show_device_menu_page_1() に失敗しました: {:?}", e);
                    })?;
            }
        }
    }
}

fn show_device_menu_page_1(device: std::sync::Arc<context::DeviceContext>) -> Result<(), pt3::Error> {
    let mut tuners: [context::TunerContext; 4] = std::array::from_fn(|index| {
        let isdb = pt3::Isdb::try_from((index / 2) as u32).unwrap_or_default();
        let tuner = (index % 2) as u32;
        
        device.create_tuner(isdb, tuner)
    });
    let mut busy = false;

    loop {
        println!("[デバイスメニュー (1 of 2)]");
        println!("0: (戻る)");
        println!("1: -> 2ページ目");
        println!("2: LNB 電源設定");
        println!("3: チャンネルスキャン");
        println!("4: チャンネル設定");
        println!("5: TS-ID 設定");
        println!("6: ステータス表示");
        println!("7: キャプチャ{}", if busy { "停止" } else { "開始"} );
        println!("8: 機能検査");

        match utility::get_number(8) {
            0 => { return Ok(()); }
            1 => {
                show_device_menu_page_2(device.clone())
                    .inspect_err(|e| {
                        eprintln!("main::show_device_menu_page_2() に失敗しました: {:?}", e);
                    })?;

            }
            2 => {
                command::set_lnb_power(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::set_lnb_power() に失敗しました: {:?}", e);
                    })?;
            }
            3 => {
                command::scan_channel(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::scan_channel() に失敗しました: {:?}", e);
                    })?;
            }
            4 => {
                command::set_channel(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::set_channel() に失敗しました: {:?}", e);
                    })?;
            }
            5 => {
                command::set_ts_id(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::set_ts_id() に失敗しました: {:?}", e);
                    })?;
            }
            6 => { 
                command::show_error_rate_count(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::show_error_rate_count() に失敗しました: {:?}", e);
                    })?;
            }
            7 => {
                if busy {
                    for tuner in &mut tuners {
                        tuner.stop();
                    }
                    busy = !busy;
                } else {
                    for tuner in &mut tuners {
                        tuner.start()
                            .inspect_err(|e| {
                                eprintln!("context::TunerContext::start() に失敗しました: {:?}", e);
                            })?;
                    }
                    busy = !busy;
                }
            }
            8 => {
                command::check_hardware(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::check_hardware() に失敗しました: {:?}", e);
                    })?;
            }
            _ => { /* do nothing */ }
        }
    }
}

fn show_device_menu_page_2(device: std::sync::Arc<context::DeviceContext>) -> Result<(), pt3::Error> {
    loop {
        println!("[デバイスメニュー (2 of 2)]");
        println!("0: (戻る)");
        println!("1: 地上アンプ電源");
        println!("2: チューナー省電力制御設定");
        println!("3: チャンネル設定×4");
        println!("4: エラッタ(FPGA 0x03)検証");

        match utility::get_number(4) {
            0 => { return Ok(()); }
            1 => {
                command::set_amp_power(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::set_amp_power() に失敗しました: {:?}", e);
                    })?;
            }
            2 => {
                command::set_tuner_sleep(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::set_tuner_sleep() に失敗しました: {:?}", e);
                    })?;
            }
            3 => {
                command::scan_test(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::scan_test() に失敗しました: {:?}", e);
                    })?;
            }
            4 => {
                command::check_eratta(device.clone())
                    .inspect_err(|e| {
                        eprintln!("command::check_eratta() に失敗しました: {:?}", e);
                    })?;
            }
            _ => { /* do nothing */ }
        }
    }
}

fn init_device(device: &std::sync::Arc<context::DeviceContext>) -> Result<(), pt3::Error> {
    device.open()
        .or_else(|e| {
            if e == pt3::Error::InvalidFpgaVersion {
                let constant_info = device.get_constant_info()
                    .inspect_err(|e| {
                        eprintln!("context::DeviceContext::get_constant_info() に失敗しました: {:?}", e);
                    })?;

                eprintln!("回路番号 {:#04x} には対応していません.", constant_info.fpga_version);
                if constant_info.fpga_version <= 0x03 {
                    eprintln!("回路を更新してください.");
                }
            }

            Err(e)
        })?;

    device.init_tuner()?;

    for isdb in pt3::Isdb::ALL {
        for tuner in 0u32..2u32 {
            device.set_tuner_sleep(isdb, tuner, false)
                .inspect_err(|e| {
                    eprintln!("context::DeviceContext::set_tuner_sleep() に失敗しました: {:?}", e);
                })?;
        }
    }

    Ok(())
}
