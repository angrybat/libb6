#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("b6/DeviceBridge.h");
        type DeviceHandle;  
        type DeviceHandleResult; 

        fn new_device() -> UniquePtr<DeviceHandleResult>;
        fn get_device_handle(self: Pin<&mut DeviceHandleResult>) -> UniquePtr<DeviceHandle>;
        fn get_error(self: &DeviceHandleResult) -> String;
    }
}

pub use ffi::*;