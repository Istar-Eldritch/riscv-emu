use crate::memory::Clocked;
use crate::memory::Memory;
pub mod clint;
pub mod flash;
pub mod plic;
pub mod rom;
pub mod uart;

pub trait Peripheral: Clocked + Memory {
    fn as_plic(&mut self) -> Option<&mut plic::PLIC> {
        None
    }
}
