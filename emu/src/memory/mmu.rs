use super::clint::CLINT;
use super::plic::PLIC;
use super::uart::{UARTDevice, UART};
use super::{Clocked, ClockedMemory, GenericMemory, Memory, MemoryError};

pub struct MMU {
    flash: GenericMemory,
    clint: CLINT,
    plic: PLIC,
    uart0: UART,
}

impl Clocked<()> for MMU {
    fn tick(&mut self, _: ()) {
        self.clint.tick(());
        self.uart0.tick(());
        self.plic.tick(&self.uart0);
    }
}

impl ClockedMemory for MMU {
    fn as_mem(&self) -> &dyn Memory {
        self
    }
    fn as_mut_mem(&mut self) -> &mut dyn Memory {
        self
    }
}

impl MMU {
    pub fn new(terminal: Option<Box<dyn UARTDevice>>) -> Self {
        Self {
            flash: GenericMemory::new(0x3_2000),
            clint: CLINT::new(),
            plic: PLIC::new(),
            uart0: UART::new(terminal),
        }
    }
    fn get_mem_mut(&mut self, addr: u32) -> Result<Box<&mut dyn Memory>, MemoryError> {
        match addr {
            v if v <= 0x0003_2000 => Ok(Box::new(&mut self.flash)), //
            v if v >= 0x0200_0000 && v < 0x200_FFFF => Ok(Box::new(&mut self.clint)),
            v if v >= 0x0C00_0000 && v < 0x1000_0000 => Ok(Box::new(&mut self.plic)),
            v if v >= 0x1001_3000 && v < 0x1001_3FFF => Ok(Box::new(&mut self.uart0)),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn get_mem(&self, addr: u32) -> Result<Box<&dyn Memory>, MemoryError> {
        match addr {
            v if v <= 0x0003_2000 => Ok(Box::new(&self.flash)), //
            v if v >= 0x0200_0000 && v < 0x200_FFFF => Ok(Box::new(&self.clint)),
            v if v >= 0x0C00_0000 && v < 0x1000_0000 => Ok(Box::new(&self.plic)),
            v if v >= 0x1001_3000 && v < 0x1001_3FFF => Ok(Box::new(&self.uart0)),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn translate_address(addr: u32) -> Result<u32, MemoryError> {
        match addr {
            v if v <= 0x0003_2000 => Ok(addr), //
            v if v >= 0x0200_0000 && v < 0x0200_FFFF => Ok(addr - 0x200_0000),
            v if v >= 0x0C00_0000 && v < 0x1000_0000 => Ok(addr - 0xC00_0000),
            v if v >= 0x1001_3000 && v < 0x1001_3FFF => Ok(addr - 0x1001_3000),
            _ => Err(MemoryError::AccessFault),
        }
    }
}

impl Memory for MMU {
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
