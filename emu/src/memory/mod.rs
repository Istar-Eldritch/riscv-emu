mod clint;
mod generic;
mod mapped_memory;
mod mmu;
mod plic;
mod uart;

pub use generic::GenericMemory;
pub use mmu::MMU;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum MemoryError {
    AccessFault,
}

pub trait Memory {
    fn tick(&mut self) -> ();

    fn rb(&self, addr: u32) -> Result<u8, MemoryError>;

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError>;

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError>;

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError>;

    fn rw(&self, addr: u32) -> Result<u32, MemoryError>;

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError>;
}
