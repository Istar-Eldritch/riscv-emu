use super::MemoryError;
use crate::Memory;
use std::ptr::{read_volatile, write_volatile};

pub struct CLINT {
    pub msip0: u32,    // addr 0
    pub mtimecmp: u64, // addr 4
    pub mtime: u64,    // addr 12
}

impl CLINT {
    pub fn new() -> Self {
        Self {
            msip0: 0,
            mtimecmp: 0,
            mtime: 0,
        }
    }

    fn translate_address(addr: u32) -> Result<usize, MemoryError> {
        match addr {
            v if v < 4 => Ok(0 + v as usize),
            v if v >= 0x4000 && v < 0x4008 => Ok(4 + (v - 0x4000) as usize),
            v if v >= 0xbff8 && v < (0xbff8 + 8) => Ok(12 + (v - 0xbff8) as usize),
            _ => Err(MemoryError::AccessFault),
        }
    }
}

impl Memory for CLINT {
    fn size(&self) -> u32 {
        0xbff8 + 8
    }

    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        let shift = Self::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *const u8 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        })
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        let shift = Self::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *mut u8 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        })
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        let shift = Self::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *const u16 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        })
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        let shift = Self::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *mut u16 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        })
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        let shift = Self::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *const u32 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        })
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        let shift = Self::translate_address(addr)?;
        let ptr: *const Self = self;
        Ok(unsafe {
            let to_read: *mut u32 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        })
    }
}
