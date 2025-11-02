#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("b6/DeviceBridge.h");
        type DeviceHandle;   
        type SysInfo;

        fn new_device() -> UniquePtr<DeviceHandle>;
        
        fn get_sys_info(self: &DeviceHandle) -> UniquePtr<SysInfo>;
    }
}

pub use ffi::*;