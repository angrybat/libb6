#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("b6/DeviceBridge.h");
        type DeviceHandle;   

        fn new_device() -> UniquePtr<DeviceHandle>;
    }
}

pub use ffi::*;