#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("b6/DeviceBridge.h");
        include!("b6/PacketBridge.h");
        type DeviceHandle;  
        type DeviceHandleResult; 

        fn new_device() -> UniquePtr<DeviceHandleResult>;
        fn get_device_handle(self: Pin<&mut DeviceHandleResult>) -> UniquePtr<DeviceHandle>;
        fn get_error(self: &DeviceHandleResult) -> String;
        fn core_type(self: &DeviceHandle) -> String;
        fn upgrade_type(self: &DeviceHandle) -> i32;
        fn language_id(self: &DeviceHandle) -> i32;
        fn customer_id(self: &DeviceHandle) -> i32;
        fn hw_version(self: &DeviceHandle) -> f64;
        fn sw_version(self: &DeviceHandle) -> f64;
        fn is_encrypted(self: &DeviceHandle) -> bool;
        fn cell_count(self: &DeviceHandle) -> i32;
        fn get_dev_info_command() -> i32;
        fn get_dev_info_command_buffer() -> Vec<u8>;
    }
}

pub use ffi::*;