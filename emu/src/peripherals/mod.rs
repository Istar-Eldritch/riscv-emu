use crate::memory::Clocked;
use crate::memory::Memory;
use std::any::Any;
pub mod clint;
pub mod flash;
pub mod plic;
pub mod uart;

pub trait Peripheral: Clocked + Memory + Any {
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct PeripheralWrapper<T>(T);

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

impl<'a, T: 'static> TryFrom<&'a mut dyn Peripheral> for PeripheralWrapper<&'a mut T> {
    type Error = ();
    fn try_from(p: &'a mut dyn Peripheral) -> Result<PeripheralWrapper<&'a mut T>, Self::Error> {
        let any: &mut dyn Any = p.as_any();
        any.downcast_mut().map(|v| PeripheralWrapper(v)).ok_or(())
    }
}
