use crate::instructions::Interrupt;
use crate::memory::Clocked;
use crate::memory::Memory;
use std::any::Any;
pub mod clint;
pub mod flash;
pub mod plic;
pub mod uart;

type RegisterInterrupt = fn(Interrupt, u32) -> ();

pub trait Peripheral: Clocked<RegisterInterrupt> + Memory + AToAny {}

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct PeripheralWrapper<T>(T);

impl<T> std::ops::Deref for PeripheralWrapper<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::DerefMut for PeripheralWrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<'a, T: 'a> TryFrom<&'a Box<dyn Peripheral + 'a>> for &'a PeripheralWrapper<T> {
    type Error = ();
    fn try_from(p: &'a Box<dyn Peripheral>) -> Result<&'a PeripheralWrapper<T>, Self::Error> {
        let any: &dyn Any = p.as_any();
        any.downcast_ref().ok_or(())
    }
}
