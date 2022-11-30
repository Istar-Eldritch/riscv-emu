use super::MemoryError;
use crate::Memory;
use std::ptr::{read_volatile, write_volatile};

pub trait TMappedMemory {
    fn translate_address(addr: u32) -> Result<usize, MemoryError>;
}

pub struct MappedMemory<T: TMappedMemory>(T);

impl<T: TMappedMemory> MappedMemory<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T: TMappedMemory> Memory for MappedMemory<T> {
    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        let shift = T::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *const u8 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        })
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        let shift = T::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *mut u8 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        })
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        let shift = T::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *const u16 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        })
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        let shift = T::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *mut u16 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        })
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        let shift = T::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *const u32 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        })
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        let shift = T::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *mut u32 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        })
    }
}
