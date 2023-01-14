use super::Memory;
use super::MemoryError;
use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

pub struct GenericMemory {
    layout: Layout,
    size: u32, // size in bytes
    addr: *mut u8,
}

impl GenericMemory {
    pub fn new(size: u32) -> Self {
        let layout = Layout::from_size_align(size as usize, 4).unwrap();
        let addr = unsafe { alloc::alloc(layout) };
        Self { size, addr, layout }
    }

    fn validate(&self, addr: u32, len: u32) -> Result<(), MemoryError> {
        if addr > self.size - len {
            return Err(MemoryError::AccessFault);
        }
        Ok(())
    }
}

impl Drop for GenericMemory {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.addr, self.layout) }
    }
}

impl Memory for GenericMemory {
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
