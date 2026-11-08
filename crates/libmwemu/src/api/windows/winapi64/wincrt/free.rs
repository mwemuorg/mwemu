use super::release;
use crate::emu;

pub(super) fn free(emu: &mut emu::Emu) {
    let p = emu.regs().rcx;
    log_red!(emu, "wincrt!free 0x{:x}", p);
    release(emu, p);
    emu.regs_mut().rax = 0;
}
