use crate::memory::{GenericMemory, Memory};
pub mod clint;
pub mod plic;
pub mod uart;

pub enum Peripheral {
    PLIC(plic::PLIC),
    CLINT(clint::CLINT),
    UART(uart::UART),
    FLASH(GenericMemory),
}

impl std::ops::Deref for Peripheral {
    type Target = dyn Memory;
    fn deref(&self) -> &(dyn Memory + 'static) {
        use Peripheral::*;
        match self {
            PLIC(p) => p,
            CLINT(c) => c,
            UART(u) => u,
            FLASH(f) => f,
        }
    }
}

impl std::ops::DerefMut for Peripheral {
    fn deref_mut(&mut self) -> &mut (dyn Memory + 'static) {
        use Peripheral::*;
        match self {
            PLIC(p) => p,
            CLINT(c) => c,
            UART(u) => u,
            FLASH(f) => f,
        }
    }
}
