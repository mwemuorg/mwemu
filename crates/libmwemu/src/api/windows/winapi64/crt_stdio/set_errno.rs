use super::set_errno;
use crate::emu;

pub(super) fn _set_errno(emu: &mut emu::Emu) {
    set_errno(emu, emu.regs().rcx as u32);
    emu.regs_mut().rax = 0;
}
