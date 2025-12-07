use std::fmt;

pub trait Message:  Send + Sync + fmt::Debug {
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct DevInfoMessage {
    pub core_type: [u8; 6],
    pub upgrade_type: u8,
    pub is_encrypted: u8,
    pub customer_id: u16,
    pub language_id: u8,
    pub sw_major: u8,
    pub sw_minor: u8,
    pub hw_version: u8,
}

impl Message for DevInfoMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(14);
        out.extend_from_slice(&[0, 0, 0, 0, 0]);
        out.extend_from_slice(&self.core_type);

        out.push(self.upgrade_type);
        out.push(self.is_encrypted);
        out.extend_from_slice(&self.customer_id.to_be_bytes());
        out.push(self.language_id);
        out.push(self.sw_major);
        out.push(self.sw_minor);
        out.push(self.hw_version);

        out
    }
}