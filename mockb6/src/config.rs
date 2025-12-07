use std::sync::Mutex;
use libc::{c_int};
use crate::messages::Message;

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
    pub reads: Vec<Box<dyn Message>>,
    pub writes: Vec<Vec<u8>>,           
    read_idx: usize,                
}

impl MockConfig {
    pub fn get_next_read(&mut self, len: usize) -> Vec<u8> {
        let idx = if self.read_idx < self.reads.len() {self.read_idx} else {self.reads.len() - 1};
        let msg = &self.reads[idx];
        self.read_idx += 1;
        let mut v = msg.serialize();
        if v.len() < len {
            v.resize(len, 0);
        }
        return v;
    }

    pub fn record_write(&mut self, data: &[u8]) {
        self.writes.push(data.to_vec());
    }
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
            reads: Vec::new(),
            writes: Vec::new(),
            read_idx: 0,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref MOCK: Mutex<MockConfig> = Mutex::new(MockConfig::default());
}
