use super::Memory;
use super::MemoryError;
use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

pub struct GenericMemory {
    layout: Layout,
    size: u32, // size in bytes
    addr: *mut u8,
    read_only: bool
}

impl GenericMemory {
    pub fn new(size: u32) -> Self {
        let layout = Layout::from_size_align(size as usize, 4).unwrap();
        let addr = unsafe { alloc::alloc(layout) };
        Self { size, addr, layout, read_only: false }
    }

    pub fn from_raw_parts(addr: *mut u8, size: u32, layout: Layout) -> Self {
        Self {
            addr,
            size,
            layout,
            read_only: false
        }
    }
    
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    fn validate(&self, addr: u32, len: u32) -> Result<(), MemoryError> {
        if addr > self.size - len {
            return Err(MemoryError::AccessFault);
        }
        Ok(())
    }
}

impl From<Vec<u8>> for GenericMemory {
    fn from(v: Vec<u8>) -> Self {
        let mut v = std::mem::ManuallyDrop::new(v);
        let addr = v.as_mut_ptr();
        let size = v.len() as u32;
        let layout = alloc::Layout::new::<Vec<u8>>();
        Self {
            addr,
            size,
            layout,
            read_only: false,
        }
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
        if self.read_only {
            return Err(MemoryError::AccessFault)
        }
        self.validate(addr, 1)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u8, value) })
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        self.validate(addr, 2)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u16) })
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        if self.read_only {
            return Err(MemoryError::AccessFault)
        }
        self.validate(addr, 2)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u16, value) })
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        self.validate(addr, 4)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u32) })
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        if self.read_only {
            return Err(MemoryError::AccessFault)
        }
        self.validate(addr, 4)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u32, value) })
    }
}
