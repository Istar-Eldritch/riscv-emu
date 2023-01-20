use super::{Exception, ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use riscv_isa_types::rv32i::*;
// test

impl Instruction for EBREAK {
    fn execute(&self, _mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        Err(ExceptionInterrupt::Exception(Exception::Breakpoint))
    }
}

impl Instruction for ECALL {
    fn execute(&self, _mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        // TODO: Environemnt calls on other privilege modes
        Err(ExceptionInterrupt::Exception(Exception::MEnvironmentCall))
    }
}

impl Instruction for CSRRCI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let t = match mcu.cpu.get_csr(self.imm) {
            Ok(v) => v,
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        };
        match mcu
            .cpu
            .set_csr(self.imm, t & !(mcu.cpu.get_x(self.rs1) & 0b11111))
        {
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
            _ => {}
        };
        mcu.cpu.set_x(self.rd, t as u32);
        Ok(1)
    }
}

impl Instruction for CSRRSI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let t = match mcu.cpu.get_csr(self.imm) {
            Ok(v) => v,
            Err(err) => {
                return Err(ExceptionInterrupt::Exception(err));
            }
        };
        let new_value = mcu.cpu.get_x(self.rs1) | t;
        match mcu.cpu.set_csr(self.imm, new_value) {
            Err(err) => {
                return Err(ExceptionInterrupt::Exception(err));
            }
            _ => {}
        };
        mcu.cpu.set_x(self.rd, t as u32);
        Ok(1)
    }
}

impl Instruction for CSRRWI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            match mcu.cpu.get_csr(self.imm) {
                Ok(v) => v,
                Err(err) => return Err(ExceptionInterrupt::Exception(err)),
            },
        );
        match mcu.cpu.set_csr(self.imm, mcu.cpu.get_x(self.rs1) & 0b11111) {
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
            _ => (),
        };
        Ok(1)
    }
}

impl Instruction for CSRRC {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let t = match mcu.cpu.get_csr(self.imm) {
            Ok(v) => v,
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        };
        match mcu.cpu.set_csr(self.imm, t & !mcu.cpu.get_x(self.rs1)) {
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
            _ => {}
        };
        mcu.cpu.set_x(self.rd, t as u32);
        Ok(1)
    }
}

impl Instruction for CSRRS {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let t = match mcu.cpu.get_csr(self.imm) {
            Ok(v) => v,
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        };
        match mcu.cpu.set_csr(self.imm, mcu.cpu.get_x(self.rs1) | t) {
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
            _ => {}
        };
        mcu.cpu.set_x(self.rd, t as u32);
        Ok(1)
    }
}

impl Instruction for CSRRW {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let t = match mcu.cpu.get_csr(self.imm) {
            Ok(v) => v,
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        };
        match mcu.cpu.set_csr(self.imm, mcu.cpu.get_x(self.rs1)) {
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
            _ => {}
        };
        mcu.cpu.set_x(self.rd, t as u32);
        Ok(1)
    }
}

impl Instruction for System {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            System::ECALL(i) => i.execute(mcu),
            System::EBREAK(i) => i.execute(mcu),
            System::CSRRW(i) => i.execute(mcu),
            System::CSRRS(i) => i.execute(mcu),
            System::CSRRC(i) => i.execute(mcu),
            System::CSRRWI(i) => i.execute(mcu),
            System::CSRRSI(i) => i.execute(mcu),
            System::CSRRCI(i) => i.execute(mcu),
        }
    }
}
