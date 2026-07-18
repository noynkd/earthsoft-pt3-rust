use earthsoft_core::pt3;
use earthsoft_core::{ Client, Tuner, Isdb, Channel, Error };
use std::io::Write;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("earthsoft-core version {}: core-pt3-example", env!("CARGO_PKG_VERSION"));

    let mut config_path = std::env::current_exe()
        .inspect_err(|e| {
            eprintln!("std::env::current_exe() に失敗しました。: {:?}", e);
        })?;
    config_path.pop();
    config_path.push("config.example.toml");

    let client = pt3::Client::new(config_path)
        .await
        .inspect_err(|e| {
            eprintln!("pt3::Client::new() に失敗しました。: {:?}", e);
        })?;

    let tuner_info = client.get_tuner_info()
        .await
        .inspect_err(|e| {
            eprintln!("pt3::Client::new() に失敗しました。: {:?}", e);
        })?;

    loop {
        println!("[チューナー選択メニュー]");
        println!("0: 終了");
        for (index, info) in tuner_info.iter().enumerate() {
            println!("{}: {}", index + 1, info.tuner_name);
        }

        match get_number(tuner_info.len() as u32) {
            0 => break,
            number => {
                let name = &tuner_info[(number - 1) as usize].tuner_name;
                let tuner = client.get_tuner(name)
                    .await
                    .inspect_err(|e| {
                        eprintln!("pt3::Client::get_tuner() に失敗しました。: {:?}", e);
                    })?;
                show_tuner_menu(tuner)
                    .await
                    .inspect_err(|e| {
                        eprintln!("pt3::Client::get_tuner() に失敗しました。: {:?}", e);
                    })?;
            }
        }
    }

    Ok(())
}

async fn show_tuner_menu(tuner: Box<dyn Tuner>) -> Result<(), Box<dyn std::error::Error>> {
    let mut in_streaming = false;

    loop {
        println!("[チューナーメニュー]");
        println!("0: 戻る");
        println!("1: チューナー情報取得");
        println!("2: 利用可能チャンネル表示");
        println!("3: 選択チャンネル表示");
        println!("4: チャンネル設定");
        println!("5: 録画{}", if in_streaming { "終了" } else { "開始" });

        match get_number(5) {
            0 => break,
            1 => {
                let info = tuner.get_info()
                    .await
                    .inspect_err(|e| {
                        eprintln!("pt3::Tuner::get_info() に失敗しました。: {:?}", e);
                    })?;
                
                println!("### チューナー情報 ###");
                println!("- チューナー名: {}", info.tuner_name);
                println!("- 　デバイス名: {}", info.device_name);
                println!("- 　　　　ISDB: {}", if info.isdb == Isdb::Satellite { "ISDB-S" } else { "ISDB-T" });
                println!("- 　　信号強度: {:5.2}", info.signal_level);
            },
            2 => {
                let channels = tuner.get_channels()
                    .await
                    .inspect_err(|e| {
                        eprintln!("pt3::Tuner::get_channels() に失敗しました。: {:?}", e);
                    })?;
                
                println!("### 利用可能チャンネル ###");
                for channel in &channels {
                    show_channel(channel);
                }
            },
            3 => {
                let channel = tuner.get_current_channel()
                    .await
                    .inspect_err(|e| {
                        eprintln!("pt3::Tuner::get_current_channel() に失敗しました。: {:?}", e);
                    })?;

                println!("### 選択中のチャンネル ###");
                show_channel(&channel);
            },
            4 => {
                let channels = tuner.get_channels()
                    .await
                    .inspect_err(|e| {
                        eprintln!("pt3::Tuner::get_channels() に失敗しました。: {:?}", e);
                    })?;
                
                let max_channel = channels.len() as u32;

                println!("[チャンネル選択]");
                println!("0: 戻る");
                println!("チャンネルを選択してください: 1-{}", max_channel);
                
                match get_number(max_channel) {
                    0 => continue,
                    number => {
                        let channel = channels.iter()
                            .find(|c| c.index == number -1)
                            .ok_or_else(|| {
                                eprintln!("チャンネルを取得できませんでした。");
                                Error::Failure
                            })?;
                        
                        tuner.set_channel(channel)
                            .await
                            .inspect_err(|e| {
                                eprintln!("pt3::Tuner::set_channel() に失敗しました。: {:?}", e);
                            })?;
                    },
                }
            },
            5 => {
                if in_streaming {
                    tuner.stop_stream()
                        .await
                        .inspect_err(|e| {
                            eprintln!("pt3::Tuner::stop_stream() に失敗しました。: {:?}", e);
                        })?;

                    in_streaming = false;
                } else {
                    let mut packet_stream = tuner.start_stream()
                        .await
                        .inspect_err(|e| {
                            eprintln!("pt3::Tuner::stop_stream() に失敗しました。: {:?}", e);
                        })?;

                    tokio::spawn(async move {
                        // 非同期ファイル書き込み用に Tokio の File を開く
                        match tokio::fs::File::create("recorded.ts").await {
                            Ok(mut file) => {
                                use tokio::io::AsyncWriteExt;
                                let mut packet_count: u64 = 0;

                                // ライブラリ側からパケット（Vec<u8>）が届く限りループする
                                while let Some(packet_result) = packet_stream.next().await {
                                    match packet_result {
                                        Ok(packet) => {
                                            // ファイルへ非同期書き込み
                                            if let Err(e) = file.write_all(&packet).await {
                                                eprintln!("\n[Error] ファイル書き込みに失敗しました: {:?}", e);
                                                break;
                                            }
                                            packet_count += 1;
                                            if packet_count % 5000 == 0 {
                                                std::print!(".");
                                                let _ = std::io::stdout().flush();
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("\n[Error] ストリームエラーが発生しました: {:?}", e);
                                            break;
                                        }
                                    }
                                }
                                // 終了時のクリーンアップ
                                let _ = file.flush().await;
                                println!("\n[Info] 録画タスクが正常に終了しました。（総パケット数: {}）", packet_count);
                            }
                            Err(e) => {
                                eprintln!("\n[Error] 録画ファイルの作成に失敗しました: {:?}", e);
                            }
                        }
                    });
                    
                    in_streaming = true;
                }
            },
            _ => continue,
        }
    }

    Ok(())
}

fn get_number(max: u32) -> u32 {
    loop {
        std::print!(">");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(num) => {
                if num <= max {
                    return num;
                }
            }
            Err(_) => {
            }
        }
    }
}

fn show_channel(channel: &Channel) {
    let space_name = match  channel.space {
        0 => "BS   ",
        1 => "CS110",
        2 => "UHF  ",
        3 => "CATV ",
        4 => "VHF  ",
        _ => "?    ",
    };

    println!("{} {}: {}", space_name, channel.index + 1, channel.name);
}
