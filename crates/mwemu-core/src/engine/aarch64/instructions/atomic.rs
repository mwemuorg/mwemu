//! ARMv8.1 LSE atomics: LD<op>, SWP, CAS and CASP. The acquire/release
//! variants (the u8 payload of the opcode) need no extra work on one core.
use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

#[derive(Clone, Copy)]
pub enum RmwOp {
    Add,
    Clr,
    Eor,
    Set,
    Smax,
    Smin,
    Umax,
    Umin,
    Swp,
}

fn sign_extend(val: u64, bytes: u64) -> i64 {
    let shift = 64 - bytes * 8;
    ((val << shift) as i64) >> shift
}

fn width_mask(bytes: u64) -> u64 {
    if bytes >= 8 {
        u64::MAX
    } else {
        (1u64 << (bytes * 8)) - 1
    }
}

fn combine(op: RmwOp, old: u64, operand: u64, bytes: u64) -> u64 {
    let (so, sv) = (sign_extend(old, bytes), sign_extend(operand, bytes));
    match op {
        RmwOp::Add => old.wrapping_add(operand),
        RmwOp::Clr => old & !operand,
        RmwOp::Eor => old ^ operand,
        RmwOp::Set => old | operand,
        RmwOp::Smax => so.max(sv) as u64,
        RmwOp::Smin => so.min(sv) as u64,
        RmwOp::Umax => old.max(operand),
        RmwOp::Umin => old.min(operand),
        RmwOp::Swp => operand,
    }
}

/// LD<op>/SWP Rs, Rt, [Xn]: [Xn] = op([Xn], Rs); Rt = old [Xn].
/// `bytes` = None uses the register width.
pub fn rmw(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>, op: RmwOp) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[0]));
    let mask = width_mask(bytes);
    let operand = read_reg(emu, &ins.operands[0]) & mask;
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[2]);
    let Some(old) = read_mem(emu, addr, bytes) else {
        log::warn!("atomic: cannot read 0x{:x}", addr);
        return false;
    };
    let new = combine(op, old, operand, bytes) & mask;
    if !write_mem(emu, addr, bytes, new) {
        return false;
    }
    write_reg(emu, &ins.operands[1], old);
    true
}

/// CAS{B,H} Rs, Rt, [Xn]: if [Xn] == Rs then [Xn] = Rt; Rs = old [Xn].
pub fn cas(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[0]));
    let mask = width_mask(bytes);
    let expected = read_reg(emu, &ins.operands[0]) & mask;
    let new = read_reg(emu, &ins.operands[1]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[2]);
    let Some(old) = read_mem(emu, addr, bytes) else {
        log::warn!("cas: cannot read 0x{:x}", addr);
        return false;
    };
    if old == expected && !write_mem(emu, addr, bytes, new) {
        return false;
    }
    write_reg(emu, &ins.operands[0], old);
    true
}

/// CASP Rs, Rs+1, Rt, Rt+1, [Xn]: 2-register compare-and-swap.
pub fn casp(emu: &mut Emu, ins: &Instruction) -> bool {
    let (Operand::RegisterPair(sz, s), Operand::RegisterPair(_, t)) =
        (ins.operands[0], ins.operands[1])
    else {
        return false;
    };
    let pair = |r: u16| {
        (
            Operand::Register(sz, r),
            Operand::Register(sz, (r + 1) & 31),
        )
    };
    let ((s0, s1), (t0, t1)) = (pair(s), pair(t));
    let bytes = reg_bytes(&s0);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[2]);
    let (Some(old0), Some(old1)) = (
        read_mem(emu, addr, bytes),
        read_mem(emu, addr + bytes, bytes),
    ) else {
        log::warn!("casp: cannot read 0x{:x}", addr);
        return false;
    };
    if old0 == read_reg(emu, &s0) && old1 == read_reg(emu, &s1) {
        let (n0, n1) = (read_reg(emu, &t0), read_reg(emu, &t1));
        if !write_mem(emu, addr, bytes, n0) || !write_mem(emu, addr + bytes, bytes, n1) {
            return false;
        }
    }
    write_reg(emu, &s0, old0);
    write_reg(emu, &s1, old1);
    true
}
