use crate::interrupt_controller::InterruptController;
use crate::memory::{Clocked, GenericMemory, Memory, MemoryError};
use crate::peripherals::Peripheral;

pub struct ROM(GenericMemory);

impl ROM {
    pub fn new(size: u32) -> Self {
        let mut mem = GenericMemory::new(size);
        mem.set_read_only(true);
        Self(mem)
    }
}

impl From<Vec<u8>> for ROM {
    fn from(v: Vec<u8>) -> Self {
        let mut memory: GenericMemory = v.into();
        memory.set_read_only(true);
        Self(memory)
    }
}

impl From<GenericMemory> for ROM {
    fn from(mut v: GenericMemory) -> Self {
        v.set_read_only(true);
        Self(v)
    }
}

impl Peripheral for ROM {}

impl Clocked for ROM {
    fn tick(&mut self, _: &mut InterruptController) {}
}
impl Memory for ROM {
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

