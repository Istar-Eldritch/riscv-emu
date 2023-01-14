use crate::instructions::{Exception, Interrupt};

pub struct CPU {
    // program counter
    pub pc: u32,
    // x regisers, ignoring x0
    x: [u32; 32],
    // waiting for interrupt
    pub wfi: bool,
    // csr registers
    csr: [u32; 8], // TODO: Implement only the CSRs I want.
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum CSRs {
    mstatus = 0x300,
    mip = 0x344,
    mie = 0x304,
    mcause = 0x342,
    mtvec = 0x305,
    mtval = 0x343,
    mepc = 0x341,
    mscratch = 0x340,
    // TODO: Supervisor mode
    // sepc = 0
    // sstatus = 0,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            x: [0; 32],
            csr: [0; 8],
            wfi: false,
        }
    }

    pub fn log_registers(&self) {
        if log::log_enabled!(log::Level::Trace) {
            let csrs = self.csr.map(|c| format!("{c:b}"));
            let x = self.x.map(|x| format!("{x:x}"));
            let pc = self.pc;
            log::trace!("executing - : csr: {csrs:?}, pc: {pc:x}, x: {x:?}");
        }
    }

    fn csr_idx_map(v: u32) -> Result<usize, Exception> {
        let m = match v {
            _ if CSRs::mstatus as u32 == v => 0,
            _ if CSRs::mip as u32 == v => 1,
            _ if CSRs::mie as u32 == v => 2,
            _ if CSRs::mcause as u32 == v => 3,
            _ if CSRs::mtvec as u32 == v => 4,
            _ if CSRs::mtval as u32 == v => 5,
            _ if CSRs::mepc as u32 == v => 6,
            _ if CSRs::mscratch as u32 == v => 7,
            _ => return Err(Exception::IllegalInstruction),
        };
        Ok(m)
    }

    pub fn get_csr(&self, addr: u32) -> Result<u32, Exception> {
        let idx = Self::csr_idx_map(addr)?;
        Ok(self.csr[idx])
    }

    pub fn set_csr(&mut self, addr: u32, v: u32) -> Result<(), Exception> {
        let idx = Self::csr_idx_map(addr)?;
        self.csr[idx] = v;
        Ok(())
    }

    pub fn get_x(&self, idx: u32) -> u32 {
        self.x[idx as usize]
    }

    pub fn set_x(&mut self, idx: u32, val: u32) {
        if idx > 0 {
            self.x[idx as usize] = val;
        }
    }

    /// Checks the mie and mip CSRs, if there are pending interrupts. Map them to a value of the
    /// [`Interrupt`] enum
    pub fn get_interrupt(&mut self) -> Option<Interrupt> {
        use Interrupt::*;
        let mie = self.get_csr(CSRs::mie as u32).unwrap();
        let mie_msi = (mie & (1 << MSoftInterrupt as u32)) != 0;
        let mie_mti = mie & (1 << MTimerInterrupt as u32) != 0;
        let mie_mei = mie & (1 << MExternalInterrupt as u32) != 0;

        let mip = self.get_csr(CSRs::mip as u32).unwrap();
        let mip_msi = mip & (1 << MSoftInterrupt as u32) != 0;
        let mip_mti = mip & (1 << MTimerInterrupt as u32) != 0;
        let mip_mei = mip & (1 << MExternalInterrupt as u32) != 0;

        match mip {
            // TODO: Handle interrupts for other privilege modes
            _v if mie_msi && mip_msi => {
                self.set_csr(CSRs::mip as u32, mip & !(1 << MSoftInterrupt as u32))
                    .unwrap();
                Some(MSoftInterrupt)
            }
            _v if mie_mti && mip_mti => Some(MTimerInterrupt),
            _v if mie_mei && mip_mei => Some(MExternalInterrupt),
            _ => None,
        }
    }
}
