use super::MemoryError;
use crate::Memory;
use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

pub struct GenericMemory {
    size: u32, // size in bytes
    addr: *mut u8,
}

impl GenericMemory {
    // TODO Implement DROP
    pub fn new(size: u32) -> Self {
        let addr = unsafe { alloc::alloc(Layout::from_size_align(size as usize, 4).unwrap()) };
        Self { size, addr }
    }

    fn validate(&self, addr: u32, len: u32) -> Result<(), MemoryError> {
        if addr > self.size - len {
            return Err(MemoryError::AccessFault);
        }
        Ok(())
    }
}

// TODO: Memory should be safe to access. Check out of bounds
impl Memory for GenericMemory {
    fn size(&self) -> u32 {
        self.size
    }

    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        self.validate(addr, 1)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u8) })
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        self.validate(addr, 1)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u8, value) })
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        self.validate(addr, 2)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u16) })
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        self.validate(addr, 2)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u16, value) })
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        self.validate(addr, 4)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u32) })
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        self.validate(addr, 4)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u32, value) })
    }
}
