use super::mapped_memory::TMappedMemory;
use super::MemoryError;

pub struct CLINT {
    pub msip0: u32,    // addr 0
    pub mtimecmp: u64, // addr 4
    pub mtime: u64,    // addr 12
}

impl CLINT {
    pub fn new() -> Self {
        Self {
            msip0: 0,
            mtimecmp: 0,
            mtime: 0,
        }
    }
}

impl TMappedMemory for CLINT {
    fn translate_address(addr: u32) -> Result<usize, MemoryError> {
        match addr {
            v if v < 4 => Ok(0 + v as usize),
            v if v >= 0x4000 && v < 0x4008 => Ok(4 + (v - 0x4000) as usize),
            v if v >= 0xbff8 && v < (0xbff8 + 8) => Ok(12 + (v - 0xbff8) as usize),
            _ => Err(MemoryError::AccessFault),
        }
    }
}
