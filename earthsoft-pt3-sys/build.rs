fn main() {
    println!("cargo:rerun-if-changed=native/include");
    println!("cargo:rerun-if-changed=native/src");

    cc::Build::new()
        .cpp(true)
        .file("native/src/pt3_bus.cpp")
        .file("native/src/pt3_device.cpp")
        .file("native/src/pt3.cpp")
        .include("../earthsoft-pt3-lib/include")
        .include("native/include")
        .include("native/src")
        .flag_if_supported("/W4")
        .flag_if_supported("/utf-8")
        .flag_if_supported("/std:c++latest")
        // .flag_if_supported("/std:c++23preview")
        // .flag_if_supported("/std:c++20")
        .compile("earthsoft_pt3");
}
