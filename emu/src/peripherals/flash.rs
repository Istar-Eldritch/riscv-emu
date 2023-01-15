use crate::memory::{Clocked, GenericMemory, Memory, MemoryError};
use crate::peripherals::{Peripheral, RegisterInterrupt};
pub struct Flash(GenericMemory);

impl Flash {
    pub fn new(size: u32) -> Self {
        Self(GenericMemory::new(size))
    }
}

impl Peripheral for Flash {}

impl Clocked<RegisterInterrupt> for Flash {
    fn tick(&mut self, _: RegisterInterrupt) {}
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
