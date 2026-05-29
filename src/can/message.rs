use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanMessage {
    pub id: u32,
    pub data: [u8; 8],
    pub timestamp: u64,
}

impl CanMessage {
    pub fn new(id: u32, data: [u8; 8], timestamp: u64) -> Self {
        Self { id, data, timestamp }
    }
    
    pub fn from_bytes(id: u32, bytes: &[u8], timestamp: u64) -> Option<Self> {
        if bytes.len() > 8 {
            return None;
        }
        let mut data = [0u8; 8];
        data[..bytes.len()].copy_from_slice(bytes);
        Some(Self { id, data, timestamp })
    }
    
    pub fn data_as_bytes(&self) -> &[u8] {
        &self.data
    }
}