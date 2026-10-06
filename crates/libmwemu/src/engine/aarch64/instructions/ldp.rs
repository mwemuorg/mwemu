use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand, SIMDSizeCode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // SIMD LDP: ldp q0, q1, [addr]
    if let Operand::SIMDRegister(sz, r1) = ins.operands[0]
        && let Operand::SIMDRegister(_, r2) = ins.operands[1]
    {
        let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);
        let reg_bytes: u64 = match sz {
            SIMDSizeCode::Q => 16,
            SIMDSizeCode::D => 8,
            SIMDSizeCode::S => 4,
            _ => 16,
        };
        guard_mem(emu, addr, (reg_bytes * 2) as u32, false);
        let v1 = read_simd_from_mem(emu, addr, reg_bytes);
        let v2 = read_simd_from_mem(emu, addr + reg_bytes, reg_bytes);
        emu.regs_aarch64_mut().v[r1 as usize] = v1;
        emu.regs_aarch64_mut().v[r2 as usize] = v2;
        do_writeback(emu, wb);
        return true;
    }

    // GPR LDP
    let is64 = operand_is_64(&ins.operands[0]);
    let sz: u64 = if is64 { 8 } else { 4 };
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);
    guard_mem(emu, addr, (sz * 2) as u32, false);

    let v1 = if is64 {
        match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match emu.maps.read_dword(addr) {
            Some(v) => v as u64,
            None => return false,
        }
    };
    let v2 = if is64 {
        match emu.maps.read_qword(addr + sz) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match emu.maps.read_dword(addr + sz) {
            Some(v) => v as u64,
            None => return false,
        }
    };

    write_reg(emu, &ins.operands[0], v1);
    write_reg(emu, &ins.operands[1], v2);
    do_writeback(emu, wb);
    true
}

fn read_simd_from_mem(emu: &Emu, addr: u64, bytes: u64) -> u128 {
    let mut raw = [0u8; 16];
    for i in 0..bytes as usize {
        raw[i] = emu.maps.read_byte(addr + i as u64).unwrap_or(0);
    }
    u128::from_le_bytes(raw)
}
