use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

pub trait Memory {
    fn size(&self) -> u32;

    fn rb(&self, addr: u32) -> u8;

    fn wb(&mut self, addr: u32, value: u8);

    fn rhw(&self, addr: u32) -> u16;

    fn whw(&mut self, addr: u32, value: u16);

    fn rw(&self, addr: u32) -> u32;

    fn ww(&mut self, addr: u32, value: u32);
}

pub struct GenericMemory {
    size: u32, // size in bytes
    addr: *mut u8,
}

impl GenericMemory {
    pub fn new(size: u32) -> Self {
        let addr = unsafe { alloc::alloc(Layout::from_size_align(size as usize, 4).unwrap()) };
        Self { size, addr }
    }
}

// TODO: Memory should be safe to access. Check out of bounds
impl Memory for GenericMemory {
    fn size(&self) -> u32 {
        self.size
    }

    fn rb(&self, addr: u32) -> u8 {
        unsafe { read_volatile((self.addr as usize + addr as usize) as *const u8) }
    }

    fn wb(&mut self, addr: u32, value: u8) {
        unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u8, value) }
    }

    fn rhw(&self, addr: u32) -> u16 {
        unsafe { read_volatile((self.addr as usize + addr as usize) as *const u16) }
    }

    fn whw(&mut self, addr: u32, value: u16) {
        unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u16, value) }
    }

    fn rw(&self, addr: u32) -> u32 {
        unsafe { read_volatile((self.addr as usize + addr as usize) as *const u32) }
    }

    fn ww(&mut self, addr: u32, value: u32) {
        unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u32, value) }
    }
}
