use super::set_errno;
use crate::emu;

pub(super) fn set_errno_api(emu: &mut emu::Emu) {
    set_errno(emu, emu.regs().rcx as u32);
    emu.regs_mut().rax = 0;
}
