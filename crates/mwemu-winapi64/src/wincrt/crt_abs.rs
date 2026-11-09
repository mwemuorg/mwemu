use crate::emu;

pub(super) fn crt_abs(emu: &mut emu::Emu) {
    let val = emu.regs().rcx as i64;
    emu.regs_mut().rax = val.unsigned_abs();
}
