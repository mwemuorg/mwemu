use super::allocate_memory;
use crate::emu;

pub(super) fn malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx; // In malloc, size is the only parameter

    if size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let base = allocate_memory(emu, size).unwrap_or(0);

    log_red!(emu, "msvcrt!malloc {} =0x{:x}", size, base);

    emu.regs_mut().rax = base;
}
