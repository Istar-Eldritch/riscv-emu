mod clint;
mod generic;
mod mapped_memory;
mod mmu;
mod plic;
pub mod uart;

pub use generic::GenericMemory;
pub use mmu::*;

pub trait Clocked<T> {
    fn tick(&mut self, deps: T);
}

pub trait ClockedMemory: Clocked<()> + Memory {
    fn as_mem(&self) -> &dyn Memory;
    fn as_mut_mem(&mut self) -> &mut dyn Memory;
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
