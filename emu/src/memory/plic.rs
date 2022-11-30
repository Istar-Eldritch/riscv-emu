use super::mapped_memory::TMappedMemory;
use super::MemoryError;

pub struct PLIC {
    pub source_priority: [u32; 52], // addr 0x0
    pub pending: u64,               // addr 0xd0
    pub h0mie: u64,                 // hart0 M-Mode interrupt enables - addr 216
    pub h0mpt: u32,                 // hart0 M-Mode priority threshold - addr 224
    pub h0mcc: u32,                 // hart0 M-Mode claim/complete - addr 228
}

impl PLIC {
    pub fn new() -> Self {
        Self {
            source_priority: [0; 52],
            pending: 0,
            h0mie: 0,
            h0mpt: 0,
            h0mcc: 0,
        }
    }
}

impl TMappedMemory for PLIC {
    fn translate_address(addr: u32) -> Result<usize, MemoryError> {
        match addr {
            v if v >= 0x4 && v < 0xd4 => Ok((v - 4) as usize),
            v if v >= 0x1000 && v < 0x1008 => Ok(0xd4 + (v - 0x1000) as usize),
            v if v >= 0x2000 && v < 0x2008 => Ok(0xdc + (v - 0x2000) as usize),
            v if v >= 0x20_0000 && v < 0x20_0004 => Ok(0xe4 + (v - 0x20_0000) as usize),
            v if v >= 0x20_0004 && v < 0x20_0008 => Ok(0xe8 + (v - 0x20_0004) as usize),
            _ => Err(MemoryError::AccessFault),
        }
    }
}
