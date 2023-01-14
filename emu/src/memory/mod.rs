pub mod clint;
mod generic;
mod mapped_memory;
mod mmu;
pub mod plic;
pub mod uart;
pub use generic::GenericMemory;
pub use mmu::*;

pub trait Clocked<T> {
    fn tick(&mut self, deps: T);
}

pub trait ClockedMemory<T = ()>: Clocked<T> + Memory {
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

pub enum Device {
    PLIC(plic::PLIC),
    CLINT(clint::CLINT),
    UART(uart::UART),
    FLASH(GenericMemory),
}

impl std::ops::Deref for Device {
    type Target = dyn Memory;
    fn deref(&self) -> &(dyn Memory + 'static) {
        use Device::*;
        match self {
            PLIC(p) => p,
            CLINT(c) => c,
            UART(u) => u,
            FLASH(f) => f,
        }
    }
}

impl std::ops::DerefMut for Device {
    fn deref_mut(&mut self) -> &mut (dyn Memory + 'static) {
        use Device::*;
        match self {
            PLIC(p) => p,
            CLINT(c) => c,
            UART(u) => u,
            FLASH(f) => f,
        }
    }
}

pub type DeviceMap =
    std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, std::cell::RefCell<Device>>>>;
