use std::sync::Mutex;
use libc::{c_int};

#[derive(Debug)]
pub struct MockConfig {
    pub init_return: c_int,
    pub open_return: c_int,
    pub kernel_driver_active: c_int,
    pub detach_return: c_int,
    pub claim_return: c_int,
    pub release_return: c_int,
    pub attach_return: c_int,
    pub interrupt_return: c_int,
    pub print_debug: bool,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            init_return: 0,
            open_return: 0,
            kernel_driver_active: 0,
            detach_return: 0,
            claim_return: 0,
            release_return: 0,
            attach_return: 0,
            interrupt_return: 1,
            print_debug: false,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref MOCK: Mutex<MockConfig> = Mutex::new(MockConfig::default());
}
