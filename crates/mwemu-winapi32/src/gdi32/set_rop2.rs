use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetROP2(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let mode = arg(emu, 1);
    log_red!(emu, "gdi32!SetROP2 mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 13; // previous: R2_COPYPEN
    stdcall_ret(emu, 2);
}
