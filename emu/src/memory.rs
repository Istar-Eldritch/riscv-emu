use std::alloc::{self, Layout};
use std::ptr::{read_volatile, write_volatile};

pub trait Memory {
    fn size(&self) -> u32;

    fn rb(&self, addr: u32) -> u8;

    fn wb(&mut self, addr: u32, value: u8);

    fn rhw(&self, addr: u32) -> u16;

    fn whw(&mut self, addr: u32, value: u16);

    fn rw(&self, addr: u32) -> u32;

    fn ww(&mut self, addr: u32, value: u32);
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
    fn get_mem_mut(&mut self, addr: u32) -> Box<&mut dyn Memory> {
        match addr {
            v if v <= 0x32000 => Box::new(&mut self.flash), //
            v if v >= 0x0200_0000 && v < 0x0200_FFFF => Box::new(&mut self.clint),
            _ => {
                todo!();
            }
        }
    }

    fn get_mem(&self, addr: u32) -> Box<&dyn Memory> {
        match addr {
            v if v < 0x32000 => Box::new(&self.flash), //
            v if v >= 0x0200_0000 && v < 0x0200_FFFF => Box::new(&self.clint),
            _ => todo!(),
        }
    }

    fn translate_address(addr: u32) -> u32 {
        match addr {
            v if v < 0x32000 => addr, //
            v if v >= 0x0200_0000 && v < 0x0200_FFFF => addr - 0x0200_0000,
            _ => todo!(),
        }
    }
}

impl Memory for MMU {
    fn size(&self) -> u32 {
        self.flash.size
    }

    fn rb(&self, addr: u32) -> u8 {
        self.get_mem(addr).rb(MMU::translate_address(addr))
    }

    fn wb(&mut self, addr: u32, value: u8) {
        self.get_mem_mut(addr)
            .wb(MMU::translate_address(addr), value)
    }

    fn rhw(&self, addr: u32) -> u16 {
        self.get_mem(addr).rhw(MMU::translate_address(addr))
    }

    fn whw(&mut self, addr: u32, value: u16) {
        self.get_mem_mut(addr)
            .whw(MMU::translate_address(addr), value)
    }

    fn rw(&self, addr: u32) -> u32 {
        self.get_mem(addr).rw(MMU::translate_address(addr))
    }

    fn ww(&mut self, addr: u32, value: u32) {
        self.get_mem_mut(addr)
            .ww(MMU::translate_address(addr), value)
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

    fn translate_address(addr: u32) -> Result<usize, ()> {
        match addr {
            v if v < 4 => Ok(0 + v as usize),
            v if v >= 0x4000 && v < 0x4008 => Ok(4 + (v - 0x4000) as usize),
            v if v >= 0xbff8 && v < (0xbff8 + 8) => Ok(12 + (v - 0xbff8) as usize),
            _ => Err(()),
        }
    }
}

impl Memory for CLINT {
    fn size(&self) -> u32 {
        0xbff8 + 8
    }

    fn rb(&self, addr: u32) -> u8 {
        let shift = Self::translate_address(addr).unwrap();
        let ptr: *const Self = self;
        unsafe {
            let to_read: *const u8 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        }
    }

    fn wb(&mut self, addr: u32, value: u8) {
        let shift = Self::translate_address(addr).unwrap();
        let ptr: *const Self = self;
        unsafe {
            let to_read: *mut u8 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        }
    }

    fn rhw(&self, addr: u32) -> u16 {
        let shift = Self::translate_address(addr).unwrap();
        let ptr: *const Self = self;
        unsafe {
            let to_read: *const u16 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        }
    }

    fn whw(&mut self, addr: u32, value: u16) {
        let shift = Self::translate_address(addr).unwrap();
        let ptr: *const Self = self;
        unsafe {
            let to_read: *mut u16 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        }
    }

    fn rw(&self, addr: u32) -> u32 {
        let shift = Self::translate_address(addr).unwrap();
        let ptr: *const Self = self;
        unsafe {
            let to_read: *const u32 = std::mem::transmute((ptr as usize) + shift);
            read_volatile(to_read)
        }
    }

    fn ww(&mut self, addr: u32, value: u32) {
        let shift = Self::translate_address(addr).unwrap();
        let ptr: *const Self = self;
        unsafe {
            let to_read: *mut u32 = std::mem::transmute((ptr as usize) + shift);
            write_volatile(to_read, value)
        }
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
}

// TODO: Memory should be safe to access. Check out of bounds
impl Memory for GenericMemory {
    fn size(&self) -> u32 {
        self.size
    }

    fn rb(&self, addr: u32) -> u8 {
        unsafe { read_volatile((self.addr as usize + addr as usize) as *const u8) }
    }

    fn wb(&mut self, addr: u32, value: u8) {
        unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u8, value) }
    }

    fn rhw(&self, addr: u32) -> u16 {
        unsafe { read_volatile((self.addr as usize + addr as usize) as *const u16) }
    }

    fn whw(&mut self, addr: u32, value: u16) {
        unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u16, value) }
    }

    fn rw(&self, addr: u32) -> u32 {
        unsafe { read_volatile((self.addr as usize + addr as usize) as *const u32) }
    }

    fn ww(&mut self, addr: u32, value: u32) {
        unsafe { write_volatile((self.addr as usize + addr as usize) as *mut u32, value) }
    }
}
