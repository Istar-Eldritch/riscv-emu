mod generic;
mod mapped_memory;
mod mmu;
use crate::interrupt_controller::InterruptController;
use crate::peripherals::Peripheral;
pub use generic::GenericMemory;
pub use mmu::*;

pub trait Clocked {
    fn tick(&mut self, interrupt_ctrl: &mut InterruptController);
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum MemoryError {
    AccessFault,
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}

impl std::error::Error for MemoryError {}

pub trait Memory {
    fn rb(&self, addr: u32) -> Result<u8, MemoryError>;

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError>;

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError>;

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError>;

    fn rw(&self, addr: u32) -> Result<u32, MemoryError>;

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError>;
}

pub type DeviceMap = std::rc::Rc<
    std::cell::RefCell<std::collections::BTreeMap<String, std::cell::RefCell<Box<dyn Peripheral>>>>,
>;
