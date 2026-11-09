use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetMapMode(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetMapMode mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mode: MM_TEXT
    stdcall_ret(emu, 2);
}
