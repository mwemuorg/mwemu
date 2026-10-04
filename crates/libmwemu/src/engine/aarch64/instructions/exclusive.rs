//! Exclusive and acquire/release loads and stores. A single emulated core
//! never loses the exclusive monitor, so store-exclusive always succeeds.
use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// LDXR/LDAXR/LDAR/LDAPR{B,H} Rt, [Xn]. `bytes` = None uses the register width.
pub fn load(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[0]));
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[1]);
    let Some(val) = read_mem(emu, addr, bytes) else {
        log::warn!("exclusive load: cannot read 0x{:x}", addr);
        return false;
    };
    write_reg(emu, &ins.operands[0], val);
    true
}

/// STLR{B,H} Rt, [Xn].
pub fn store_release(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[0]));
    let val = read_reg(emu, &ins.operands[0]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[1]);
    write_mem(emu, addr, bytes, val)
}

/// STXR/STLXR{B,H} Ws, Rt, [Xn]: store and report success (Ws = 0).
pub fn store(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[1]));
    let val = read_reg(emu, &ins.operands[1]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[2]);
    if !write_mem(emu, addr, bytes, val) {
        return false;
    }
    write_reg(emu, &ins.operands[0], 0);
    true
}

/// LDXP/LDAXP Rt, Rt2, [Xn].
pub fn load_pair(emu: &mut Emu, ins: &Instruction) -> bool {
    let bytes = reg_bytes(&ins.operands[0]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[2]);
    let (Some(v1), Some(v2)) = (
        read_mem(emu, addr, bytes),
        read_mem(emu, addr + bytes, bytes),
    ) else {
        log::warn!("exclusive load pair: cannot read 0x{:x}", addr);
        return false;
    };
    write_reg(emu, &ins.operands[0], v1);
    write_reg(emu, &ins.operands[1], v2);
    true
}

/// STXP/STLXP Ws, Rt, Rt2, [Xn]: store pair and report success (Ws = 0).
pub fn store_pair(emu: &mut Emu, ins: &Instruction) -> bool {
    let bytes = reg_bytes(&ins.operands[1]);
    let v1 = read_reg(emu, &ins.operands[1]);
    let v2 = read_reg(emu, &ins.operands[2]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[3]);
    if !write_mem(emu, addr, bytes, v1) || !write_mem(emu, addr + bytes, bytes, v2) {
        return false;
    }
    write_reg(emu, &ins.operands[0], 0);
    true
}
