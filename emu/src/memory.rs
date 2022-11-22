use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

pub struct Memory {
    size: u32, // size in bytes
    addr: *mut u8,
}

// TODO: Memory should be safe to access. Check out of bounds
impl Memory {
    pub fn new(size: u32) -> Self {
        let addr = unsafe { alloc::alloc(Layout::from_size_align(size as usize, 4).unwrap()) };
        Memory { size, addr }
    }

    pub fn rb(&self, addr: usize) -> u8 {
        unsafe { read_volatile((self.addr as usize + addr) as *const u8) }
    }

    pub fn wb(&mut self, addr: usize, value: u8) {
        unsafe { write_volatile((self.addr as usize + addr) as *mut u8, value) }
    }

    pub fn rhw(&self, addr: usize) -> u16 {
        unsafe { read_volatile((self.addr as usize + addr) as *const u16) }
    }

    pub fn whw(&mut self, addr: usize, value: u16) {
        unsafe { write_volatile((self.addr as usize + addr) as *mut u16, value) }
    }

    pub fn rw(&self, addr: usize) -> u32 {
        unsafe { read_volatile((self.addr as usize + addr) as *const u32) }
    }

    pub fn ww(&mut self, addr: usize, value: u32) {
        unsafe { write_volatile((self.addr as usize + addr) as *mut u32, value) }
    }
}
