use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand, SIMDSizeCode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // SIMD LDR: ldr q0, [addr] / ldr d0, [addr] / ldr s0, [addr]
    if let Operand::SIMDRegister(sz, rd) = ins.operands[0] {
        let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
        let reg_bytes: usize = match sz {
            SIMDSizeCode::Q => 16,
            SIMDSizeCode::D => 8,
            SIMDSizeCode::S => 4,
            SIMDSizeCode::H => 2,
            SIMDSizeCode::B => 1,
        };
        guard_mem(emu, addr, reg_bytes as u32, false);
        let mut raw = [0u8; 16];
        for i in 0..reg_bytes {
            raw[i] = emu.maps.read_byte(addr + i as u64).unwrap_or(0);
        }
        emu.regs_aarch64_mut().v[rd as usize] = u128::from_le_bytes(raw);
        do_writeback(emu, wb);
        return true;
    }

    // GPR LDR
    let is64 = operand_is_64(&ins.operands[0]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
    guard_mem(emu, addr, if is64 { 8 } else { 4 }, false);
    let val = if is64 {
        match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => {
                log::warn!("LDR: cannot read 8 bytes at 0x{:x}", addr);
                return false;
            }
        }
    } else {
        match emu.maps.read_dword(addr) {
            Some(v) => v as u64,
            None => {
                log::warn!("LDR: cannot read 4 bytes at 0x{:x}", addr);
                return false;
            }
        }
    };
    write_reg(emu, &ins.operands[0], val);
    do_writeback(emu, wb);
    true
}
