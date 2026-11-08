use super::release;
use crate::emu;

pub(super) fn crt_aligned_free(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "wincrt!_aligned_free 0x{:x}", ptr);
    release(emu, ptr);
}
