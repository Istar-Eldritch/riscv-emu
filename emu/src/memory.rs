use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum MemoryError {
    AccessFault,
}

pub trait Memory {
    fn size(&self) -> u32;

    fn rb(&self, addr: u32) -> Result<u8, MemoryError>;

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError>;

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError>;

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError>;

    fn rw(&self, addr: u32) -> Result<u32, MemoryError>;

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError>;
}

pub struct MMU {
    flash: GenericMemory,
    clint: CLINT,
}

impl MMU {
    pub fn new() -> Self {
        Self {
            flash: GenericMemory::new(0x32000),
            clint: CLINT::new(),
        }
    }
    fn get_mem_mut(&mut self, addr: u32) -> Result<Box<&mut dyn Memory>, MemoryError> {
        match addr {
            v if v <= 0x32000 => Ok(Box::new(&mut self.flash)), //
            v if v >= 0x200_0000 && v < 0x200_FFFF => Ok(Box::new(&mut self.clint)),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn get_mem(&self, addr: u32) -> Result<Box<&dyn Memory>, MemoryError> {
        match addr {
            v if v < 0x32000 => Ok(Box::new(&self.flash)), //
            v if v >= 0x200_0000 && v < 0x200_FFFF => Ok(Box::new(&self.clint)),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn translate_address(addr: u32) -> Result<u32, MemoryError> {
        match addr {
            v if v < 0x32000 => Ok(addr), //
            v if v >= 0x200_0000 && v < 0x200_FFFF => Ok(addr - 0x200_0000),
            _ => Err(MemoryError::AccessFault),
        }
    }
}

impl Memory for MMU {
    fn size(&self) -> u32 {
        self.flash.size
    }

    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        self.get_mem(addr)?.rb(MMU::translate_address(addr)?)
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        self.get_mem_mut(addr)?
            .wb(MMU::translate_address(addr)?, value)
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        self.get_mem(addr)?.rhw(MMU::translate_address(addr)?)
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        self.get_mem_mut(addr)?
            .whw(MMU::translate_address(addr)?, value)
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        self.get_mem(addr)?.rw(MMU::translate_address(addr)?)
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        self.get_mem_mut(addr)?
            .ww(MMU::translate_address(addr)?, value)
    }
}

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

pub struct GenericMemory {
    size: u32, // size in bytes
    addr: *mut u8,
}

impl GenericMemory {
    // TODO Implement DROP
    pub fn new(size: u32) -> Self {
        let addr = unsafe { alloc::alloc(Layout::from_size_align(size as usize, 4).unwrap()) };
        Self { size, addr }
    }

    fn validate(&self, addr: u32, len: u32) -> Result<(), MemoryError> {
        if addr > self.size - len {
            return Err(MemoryError::AccessFault);
        }
        Ok(())
    }
}

// TODO: Memory should be safe to access. Check out of bounds
impl Memory for GenericMemory {
    fn size(&self) -> u32 {
        self.size
    }

    fn rb(&self, addr: u32) -> Result<u8, MemoryError> {
        self.validate(addr, 1)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u8) })
    }

    fn wb(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        self.validate(addr, 1)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u8, value) })
    }

    fn rhw(&self, addr: u32) -> Result<u16, MemoryError> {
        self.validate(addr, 2)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u16) })
    }

    fn whw(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        self.validate(addr, 2)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u16, value) })
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        self.validate(addr, 4)?;
        Ok(unsafe { read_volatile((self.addr as usize + addr as usize) as *const u32) })
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        self.validate(addr, 4)?;
        Ok(unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u32, value) })
    }
}
