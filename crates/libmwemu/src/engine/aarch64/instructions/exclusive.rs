//! Exclusive and acquire/release loads and stores.
//!
//! Threads interleave instruction by instruction, so the exclusive monitor is
//! modelled per thread as compare-and-swap: an exclusive load records what it
//! read, and the store-exclusive succeeds only if memory still holds it.
use crate::emu::Emu;
use crate::threading::context::Reservation;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode};

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
    if is_exclusive_load(ins.opcode) {
        reserve(emu, addr, bytes, (val, 0));
    }
    true
}

/// LDXR/LDAXR set a reservation; LDAR/LDAPR are plain acquire loads.
fn is_exclusive_load(op: Opcode) -> bool {
    matches!(
        op,
        Opcode::LDXR
            | Opcode::LDAXR
            | Opcode::LDXRB
            | Opcode::LDAXRB
            | Opcode::LDXRH
            | Opcode::LDAXRH
    )
}

fn reserve(emu: &mut Emu, addr: u64, bytes: u64, values: (u64, u64)) {
    let cur = emu.current_thread_id;
    emu.threads[cur].exclusive = Some(Reservation {
        addr,
        bytes,
        values,
    });
}

/// Consume the current thread's reservation; true if it still covers `addr`
/// and memory has not changed since the exclusive load.
fn reservation_holds(emu: &mut Emu, addr: u64, bytes: u64, pair: bool) -> bool {
    let cur = emu.current_thread_id;
    let Some(r) = emu.threads[cur].exclusive.take() else {
        return false;
    };
    if r.addr != addr || r.bytes != bytes {
        return false;
    }
    let first = read_mem(emu, addr, bytes) == Some(r.values.0);
    let second = !pair || read_mem(emu, addr + bytes, bytes) == Some(r.values.1);
    first && second
}

/// STLR{B,H} Rt, [Xn].
pub fn store_release(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[0]));
    let val = read_reg(emu, &ins.operands[0]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[1]);
    write_mem(emu, addr, bytes, val)
}

/// STXR/STLXR{B,H} Ws, Rt, [Xn]: store if the reservation holds; Ws = 0 on
/// success, 1 on failure.
pub fn store(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>) -> bool {
    let bytes = bytes.unwrap_or_else(|| reg_bytes(&ins.operands[1]));
    let val = read_reg(emu, &ins.operands[1]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[2]);
    if !reservation_holds(emu, addr, bytes, false) {
        write_reg(emu, &ins.operands[0], 1);
        return true;
    }
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
    reserve(emu, addr, bytes, (v1, v2));
    true
}

/// STXP/STLXP Ws, Rt, Rt2, [Xn]: store pair if the reservation holds;
/// Ws = 0 on success, 1 on failure.
pub fn store_pair(emu: &mut Emu, ins: &Instruction) -> bool {
    let bytes = reg_bytes(&ins.operands[1]);
    let v1 = read_reg(emu, &ins.operands[1]);
    let v2 = read_reg(emu, &ins.operands[2]);
    let (addr, _) = resolve_mem_addr(emu, &ins.operands[3]);
    if !reservation_holds(emu, addr, bytes, true) {
        write_reg(emu, &ins.operands[0], 1);
        return true;
    }
    if !write_mem(emu, addr, bytes, v1) || !write_mem(emu, addr + bytes, bytes, v2) {
        return false;
    }
    write_reg(emu, &ins.operands[0], 0);
    true
}
