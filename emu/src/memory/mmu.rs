use super::clint::CLINT;
use super::mapped_memory::MappedMemory;
use super::plic::PLIC;
use super::{GenericMemory, Memory, MemoryError};

pub struct MMU {
    flash: GenericMemory,
    clint: MappedMemory<CLINT>,
    plic: PLIC,
}

impl MMU {
    pub fn new() -> Self {
        Self {
            flash: GenericMemory::new(0x32000),
            clint: MappedMemory::new(CLINT::new()),
            plic: PLIC::new(),
        }
    }
    fn get_mem_mut(&mut self, addr: u32) -> Result<Box<&mut dyn Memory>, MemoryError> {
        match addr {
            v if v <= 0x32000 => Ok(Box::new(&mut self.flash)), //
            v if v >= 0x200_0000 && v < 0x200_FFFF => Ok(Box::new(&mut self.clint)),
            v if v >= 0xC00_0000 && v < 0x1000_0000 => Ok(Box::new(&mut self.plic)),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn get_mem(&self, addr: u32) -> Result<Box<&dyn Memory>, MemoryError> {
        match addr {
            v if v < 0x32000 => Ok(Box::new(&self.flash)), //
            v if v >= 0x200_0000 && v < 0x200_FFFF => Ok(Box::new(&self.clint)),
            v if v >= 0xC00_0000 && v < 0x1000_0000 => Ok(Box::new(&self.plic)),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn translate_address(addr: u32) -> Result<u32, MemoryError> {
        match addr {
            v if v < 0x32000 => Ok(addr), //
            v if v >= 0x200_0000 && v < 0x200_FFFF => Ok(addr - 0x200_0000),
            v if v >= 0xC00_0000 && v < 0x1000_0000 => Ok(addr - 0xC00_0000),
            _ => Err(MemoryError::AccessFault),
        }
    }
}

impl Memory for MMU {
    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        self.get_mem(addr)?.rb(MMU::translate_address(addr)?)
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        self.get_mem_mut(addr)?
            .wb(MMU::translate_address(addr)?, value)
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        self.get_mem(addr)?.rhw(MMU::translate_address(addr)?)
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        self.get_mem_mut(addr)?
            .whw(MMU::translate_address(addr)?, value)
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        self.get_mem(addr)?.rw(MMU::translate_address(addr)?)
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        self.get_mem_mut(addr)?
            .ww(MMU::translate_address(addr)?, value)
    }
}
