use crate::interrupt_controller::InterruptController;
use crate::memory::{Clocked, GenericMemory, Memory, MemoryError};
use crate::peripherals::Peripheral;

pub struct Flash(GenericMemory);

impl Flash {
    pub fn new(size: u32) -> Self {
        Self(GenericMemory::new(size))
    }
}

impl From<Vec<u8>> for Flash {
    fn from(v: Vec<u8>) -> Self {
        let mut mem: GenericMemory = v.into();
        mem.set_read_only(false);
        Self(mem)
    }
}

impl From<GenericMemory> for Flash {
    fn from(mut v: GenericMemory) -> Self {
        v.set_read_only(false);
        Self(v)
    }
}

impl Peripheral for Flash {}

impl Clocked for Flash {
    fn tick(&mut self, _: &mut InterruptController) {}
}
impl Memory for Flash {
    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        self.0.rb(addr)
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        self.0.wb(addr, value)
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        self.0.rhw(addr)
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        self.0.whw(addr, value)
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        self.0.rw(addr)
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        self.0.ww(addr, value)
    }
}

