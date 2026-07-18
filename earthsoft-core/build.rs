use std::{env, fs, path::PathBuf};

fn main() {
    // config もしくは examples が更新された場合に build.rs を実行する
    println!("cargo:rerun-if-changed=config");
    println!("cargo:rerun-if-changed=examples");

    // config.example.toml を examples に配布する
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("config/config.example.toml");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dest_path = out_path
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf();
    dest_path.push("examples");

    if dest_path.exists() {
        dest_path.push("config.example.toml");

        if let Err(e) = fs::copy(&config_path, &dest_path) {
            println!("cargo:warning=Failed to copy config.toml: {:?}", e);
        }
    }
}
