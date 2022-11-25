use super::format::*;
use crate::utils::*;
use crate::{exception, CPUException, Memory, CPU};

/// Load Upper Immediate
fn lui(cpu: &mut CPU, _mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = UFormat::try_from(word).unwrap(); // TODO: Handle trap
    cpu.x[parsed.rd as usize] = sext(parsed.imm << 12, 20, 32);
    Some(1)
}

/// Add Upper Immediate to PC
fn auipc(cpu: &mut CPU, _mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = UFormat::try_from(word).unwrap();
    cpu.x[parsed.rd as usize] = cpu.pc + sext(parsed.imm << 12, 20, 32);
    Some(1)
}

/// Jump and Link
fn jal(cpu: &mut CPU, _mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = JFormat::try_from(word).unwrap();
    cpu.x[parsed.rd as usize] = cpu.pc + 4;
    cpu.pc += sext(
        parsed.imm0 << 11 | parsed.imm1 << 10 | parsed.imm2 | parsed.imm3 << 19,
        20,
        32,
    );
    Some(1)
}

/// Jump and Link Register
fn jalr(cpu: &mut CPU, _mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = IFormat::try_from(word).unwrap();
    cpu.pc = (cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32)) & !1;
    let addr = if parsed.rd == 0 { 1 } else { parsed.rd };
    cpu.x[addr as usize] = cpu.pc + 4;
    Some(1)
}

fn beq(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat, _word: u32) -> Option<u32> {
    if cpu.x[parsed.rs1 as usize] == cpu.x[parsed.rs2 as usize] {
        cpu.pc += sext(
            parsed.imm0 << 10 | parsed.imm1 | parsed.imm2 << 4 | parsed.imm3 << 11,
            12,
            32,
        );
    }
    Some(1)
}

fn bge(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat, _word: u32) -> Option<u32> {
    if cpu.x[parsed.rs1 as usize] as i32 >= cpu.x[parsed.rs2 as usize] as i32 {
        cpu.pc += sext(
            parsed.imm0 << 10 | parsed.imm1 | parsed.imm2 << 4 | parsed.imm3 << 11,
            12,
            32,
        );
    }
    Some(1)
}

fn bgeu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat, _word: u32) -> Option<u32> {
    if cpu.x[parsed.rs1 as usize] >= cpu.x[parsed.rs2 as usize] {
        cpu.pc += sext(
            parsed.imm0 << 10 | parsed.imm1 | parsed.imm2 << 4 | parsed.imm3 << 11,
            12,
            32,
        );
    }
    Some(1)
}

fn blt(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat, _word: u32) -> Option<u32> {
    if (cpu.x[parsed.rs1 as usize] as i32) < cpu.x[parsed.rs2 as usize] as i32 {
        cpu.pc += sext(
            parsed.imm0 << 10 | parsed.imm1 | parsed.imm2 << 4 | parsed.imm3 << 11,
            12,
            32,
        );
    }
    Some(1)
}

fn bltu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat, _word: u32) -> Option<u32> {
    if cpu.x[parsed.rs1 as usize] < cpu.x[parsed.rs2 as usize] {
        cpu.pc += sext(
            parsed.imm0 << 10 | parsed.imm1 | parsed.imm2 << 4 | parsed.imm3 << 11,
            12,
            32,
        );
    }
    Some(1)
}

fn bne(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat, _word: u32) -> Option<u32> {
    if cpu.x[parsed.rs1 as usize] != cpu.x[parsed.rs2 as usize] {
        cpu.pc += sext(
            parsed.imm0 << 10 | parsed.imm1 | parsed.imm2 << 4 | parsed.imm3 << 11,
            12,
            32,
        );
    }
    Some(1)
}

/// Branch parsing
fn branch(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = BFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => beq,
        0b101 => bge,  // bge
        0b111 => bgeu, // bgeu
        0b100 => blt,  // blt
        0b110 => bltu, // bltu
        0b001 => bne,  // bne
        _ => {
            exception(cpu, CPUException::IllegalInstruction, word);
            return Some(0);
        } // TODO: Trap
    };
    op(cpu, mem, parsed, word)
}

fn lb(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = sext(mem.rb(addr) as u32, 8, 32);
    Some(1)
}

fn lbu(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = mem.rb(addr) as u32;
    Some(1)
}

fn lh(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = sext(mem.rhw(addr) as u32, 16, 32);
    Some(1)
}

fn lhu(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = mem.rhw(addr) as u32;
    Some(1)
}

fn lw(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = sext(mem.rw(addr) as u32, 32, 32);
    Some(1)
}

fn lwu(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = mem.rw(addr) as u32;
    Some(1)
}

fn load(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = IFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => lb,
        0b100 => lbu,
        0b001 => lh,
        0b101 => lhu,
        0b010 => lw,
        0b110 => lwu,
        _ => {
            exception(cpu, CPUException::IllegalInstruction, word);
            return Some(0);
        }
    };
    op(cpu, mem, parsed, word)
}

fn sb(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.wb(addr, cpu.x[parsed.rs2 as usize] as u8);
    Some(1)
}

fn sh(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.whw(addr, cpu.x[parsed.rs2 as usize] as u16);
    Some(1)
}

fn sw(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat, _word: u32) -> Option<u32> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.ww(addr, cpu.x[parsed.rs2 as usize]);
    Some(1)
}

fn store(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = SFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => sb,
        0b001 => sh,
        0b010 => sw,
        _ => {
            exception(cpu, CPUException::IllegalInstruction, word);
            return Some(0);
        }
    };
    op(cpu, mem, parsed, word)
}

fn addi(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_add(sext(parsed.imm, 12, 32));
    Some(1)
}

fn slti(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let v = if (cpu.x[parsed.rs1 as usize] as i32) < sext(parsed.imm, 12, 32) as i32 {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;
    Some(1)
}

fn sltiu(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let v = if cpu.x[parsed.rs1 as usize] < sext(parsed.imm, 12, 32) {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;
    Some(1)
}

fn xori(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize] ^ sext(cpu.x[parsed.imm as usize], 12, 32);
    Some(1)
}

fn ori(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize] | sext(cpu.x[parsed.imm as usize], 12, 32);
    Some(1)
}

fn andi(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize] & sext(cpu.x[parsed.imm as usize], 12, 32);
    Some(1)
}

fn slli(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_shl(shamt);
    Some(1)
}

fn srli(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_shr(shamt);
    Some(1)
}

fn srai(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat, _word: u32) -> Option<u32> {
    let shamt = parsed.imm & 0b11111;
    let rs1 = cpu.x[parsed.rs1 as usize];
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.x[parsed.rd as usize] = (rs1 as i32).wrapping_shr(shamt) as u32;
    Some(1)
}

fn immediate(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = IFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => addi,
        0b010 => slti,
        0b011 => sltiu,
        0b100 => xori,
        0b110 => ori,
        0b111 => andi,
        0b001 => slli,
        0b101 if parsed.imm & (0b111111 << 11) == 0 => srli,
        0b101 => srai,
        _ => {
            exception(cpu, CPUException::IllegalInstruction, word);
            return Some(0);
        }
    };
    op(cpu, mem, parsed, word)
}

fn add(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_add(cpu.x[parsed.rs2 as usize]);
    Some(1)
}

fn sub(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_sub(cpu.x[parsed.rs2 as usize]);
    Some(1)
}

fn sll(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize].wrapping_shl(cpu.x[parsed.rs2 as usize] & 0b11111);
    Some(1)
}

fn slt(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    let v = if (cpu.x[parsed.rs1 as usize] as i32) < cpu.x[parsed.rs2 as usize] as i32 {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;
    Some(1)
}

fn sltu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    let v = if cpu.x[parsed.rs1 as usize] < cpu.x[parsed.rs2 as usize] {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;
    Some(1)
}

fn xor(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize] ^ cpu.x[parsed.rs2 as usize];
    Some(1)
}

fn srl(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize].wrapping_shr(cpu.x[parsed.rs2 as usize] & 0b11111);
    Some(1)
}

fn sra(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    let shamt = cpu.x[parsed.rs2 as usize] & 0b11111;
    let rs1 = cpu.x[parsed.rs1 as usize];
    cpu.x[parsed.rd as usize] = (rs1 as i32).wrapping_shr(shamt) as u32;
    Some(1)
}

fn or(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize] | cpu.x[parsed.rs2 as usize];
    Some(1)
}

fn and(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat, _word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize] & cpu.x[parsed.rs2 as usize];
    Some(1)
}

fn arithmetic(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = RFormat::from(word);
    let op = match parsed.funct3 {
        0b000 if parsed.funct7 == 0 => add,
        0b000 => sub,
        0b001 => sll,
        0b010 => slt,
        0b011 => sltu,
        0b100 => xor,
        0b101 if parsed.funct7 == 0 => srl,
        0b101 => sra,
        0b110 => or,
        0b111 => and,
        _ => {
            exception(cpu, CPUException::IllegalInstruction, word);
            return Some(0);
        }
    };
    op(cpu, mem, parsed, word)
}

fn fence(_cpu: &mut CPU, _mem: &mut dyn Memory, _word: u32) -> Option<u32> {
    // No need to do anything given we do not reorder memroy writes / loads
    Some(0)
}

fn csrrw(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat, word: u32) -> Option<u32> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
    };
    match cpu.set_csr(parsed.imm, cpu.x[parsed.rs1 as usize]) {
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
        _ => {}
    };
    cpu.x[parsed.rd as usize] = t as u32;
    Some(1)
}

fn csrrs(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat, word: u32) -> Option<u32> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
    };
    match cpu.set_csr(parsed.imm, cpu.x[parsed.rs1 as usize] | t) {
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
        _ => {}
    };
    cpu.x[parsed.rd as usize] = t as u32;
    Some(1)
}

fn csrrc(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat, word: u32) -> Option<u32> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
    };
    match cpu.set_csr(parsed.imm, t & !cpu.x[parsed.rs1 as usize]) {
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
        _ => {}
    };
    cpu.x[parsed.rd as usize] = t as u32;
    Some(1)
}

fn csrrwi(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat, word: u32) -> Option<u32> {
    cpu.x[parsed.rd as usize] = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
    };
    match cpu.set_csr(parsed.imm, cpu.x[parsed.rs1 as usize] & 0b11111) {
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
        _ => (),
    };
    Some(1)
}

fn csrrsi(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat, word: u32) -> Option<u32> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
    };
    match cpu.set_csr(parsed.imm, cpu.x[parsed.rs1 as usize] & 0b11111 | t) {
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
        _ => {}
    };
    cpu.x[parsed.rd as usize] = t as u32;
    Some(1)
}

fn csrrci(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat, word: u32) -> Option<u32> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
    };
    match cpu.set_csr(parsed.imm, t & !(cpu.x[parsed.rs1 as usize] & 0b11111)) {
        Err(err) => {
            exception(cpu, err, word);
            return Some(0);
        }
        _ => {}
    };
    cpu.x[parsed.rd as usize] = t as u32;
    Some(1)
}

fn system(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let parsed = IFormat::from(word);

    let op = match parsed.funct3 {
        0b000 if parsed.imm == 0 => {
            exception(cpu, CPUException::UEnvironmentCall, 0);
            return Some(1);
        }
        0b000 if parsed.imm == 1 => {
            exception(cpu, CPUException::Breakpoint, cpu.pc);
            return Some(1);
        }
        0b001 => csrrw,
        0b010 => csrrs,
        0b011 => csrrc,
        0b101 => csrrwi,
        0b110 => csrrsi,
        0b111 => csrrci,
        _ => {
            return None;
        }
    };

    op(cpu, mem, parsed, word)
}

pub fn rv32i(cpu: &mut CPU, mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let opcode = opcode(word);
    let op = match opcode {
        0b0110111 => lui,
        0b0010111 => auipc,
        0b1101111 => jal,
        0b1100111 => jalr,
        0b1100011 => branch,
        0b0000011 => load,
        0b0100011 => store,
        0b0010011 => immediate,
        0b0110011 => arithmetic,
        0b0001111 => fence,
        0b1110011 => system,
        _ => {
            return None;
        }
    };

    op(cpu, mem, word)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::GenericMemory;
    #[test]
    fn lui_writes_register() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        // lui x1, 0x23
        let word = 0x0002_20b7;
        lui(&mut cpu, &mut mem, word);
        assert_eq!(cpu.x[1], 0x22 << 12);
    }
}
