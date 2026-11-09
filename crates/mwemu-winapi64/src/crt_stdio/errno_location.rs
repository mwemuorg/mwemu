use super::errno_cell;
use crate::emu;

pub(super) fn errno_location(emu: &mut emu::Emu) {
    emu.regs_mut().rax = errno_cell(emu);
}
