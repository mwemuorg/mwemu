use crate::emu;
use crate::winapi::helper;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx;

    log_red!(emu, "kernel32!SetErrorMode 0x{:x}", mode);
    helper::advance_tick(emu, 1);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // return previous error mode
}
