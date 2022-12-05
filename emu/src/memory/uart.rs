use crate::memory::Clocked;
use crate::memory::{Memory, MemoryError};
use std::cell::RefCell;
use std::collections::VecDeque;

pub trait UARTDevice {
    fn read(&mut self) -> Option<u8>;
    fn write(&mut self, byte: u8);
}

pub struct UART {
    r_fifo: RefCell<VecDeque<u8>>, // Receive fifo
    t_fifo: VecDeque<u8>,          // Send fifo
    rxctrl: u32,
    txctrl: u32,
    ie: u32,  // Interrupt enable
    ip: u32,  // Interrupt pending
    div: u32, // Baud rate divider br = clk / (br_div + 1)
    device: Option<Box<dyn UARTDevice>>,
}

impl UART {
    pub fn new(device: Option<Box<dyn UARTDevice>>) -> Self {
        Self {
            r_fifo: RefCell::new(VecDeque::new()),
            t_fifo: VecDeque::new(),
            txctrl: 0,
            rxctrl: 0,
            ie: 0,
            ip: 0,
            div: 0,
            device,
        }
    }

    fn read(&self) -> u32 {
        let mut r_fifo = self.r_fifo.borrow_mut();
        if let Some(b) = r_fifo.pop_back() {
            b as u32
        } else {
            1 << 31
        }
    }

    fn write(&mut self, v: u8) -> () {
        self.t_fifo.push_front(v)
    }
}

impl Clocked<()> for UART {
    fn tick(&mut self, _: ()) -> () {
        let mut r_fifo = self.r_fifo.borrow_mut();
        if let Some(device) = &mut self.device {
            // rxen = 1
            if self.rxctrl & 0b1 != 0 {
                if let Some(b) = device.read() {
                    r_fifo.push_front(b);
                }
            }

            // txen = 1
            if self.txctrl & 0b1 != 0 {
                if let Some(b) = self.t_fifo.pop_back() {
                    device.write(b);
                }
            }
        }

        let rxcnt = (self.rxctrl & (0b11 << 16)) >> 16;
        if r_fifo.len() > rxcnt as usize {
            self.ip = self.ip | 0b10;
        } else {
            self.ip = self.ip & !0b10;
        }

        let txcnt = (self.txctrl & (0b11 << 16)) >> 16;
        if self.t_fifo.len() < txcnt as usize {
            self.ip = self.ip | 0b01;
        } else {
            self.ip = self.ip & !0b01;
        }
    }
}

impl Memory for UART {
    fn rb(&self, _addr: u32) -> Result<u8, MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn wb(&mut self, _addr: u32, _value: u8) -> Result<(), MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn rhw(&self, _addr: u32) -> Result<u16, MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn whw(&mut self, _addr: u32, _value: u16) -> Result<(), MemoryError> {
        Err(MemoryError::AccessFault)
    }

    fn rw(&self, addr: u32) -> Result<u32, MemoryError> {
        match addr {
            0x00 => Ok(if self.t_fifo.len() < 8 { 0 } else { 1 << 31 }),
            0x04 => Ok(self.read()),
            0x08 => Ok(self.txctrl),
            0x0C => Ok(self.rxctrl),
            0x10 => Ok(self.ie),
            0x14 => Ok(self.ip),
            0x18 => Ok(self.div),
            _ => Err(MemoryError::AccessFault),
        }
    }

    fn ww(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        match addr {
            0x00 => Ok(self.write(value as u8)),
            0x04 => Ok(()),
            0x08 => {
                self.txctrl = value;
                Ok(())
            }
            0x0C => {
                self.rxctrl = value;
                Ok(())
            }
            0x10 => {
                self.ie = value;
                Ok(())
            }
            0x14 => Ok(()),
            0x18 => {
                self.div = value;
                Ok(())
            }
            _ => Err(MemoryError::AccessFault),
        }
    }
}
