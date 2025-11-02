fn main() {
    // generate and compile cxx bridge + compile DeviceBridge.cc
    cxx_build::bridge("src/lib.rs")
        .file("src/DeviceBridge.cc")
        .file("include/b6/Device.cc")
        .include("include")
        .include("/usr/include/libusb-1.0")
        .flag_if_supported("-std=c++14")
        .compile("device_bridge");

    // link libusb (adjust name/path if needed on Windows)
    println!("cargo:rustc-link-lib=usb-1.0");
    println!("cargo:rerun-if-changed=src/DeviceBridge.cc");
    println!("cargo:rerun-if-changed=include/b6/DeviceBridge.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
}