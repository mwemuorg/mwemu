use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetTextAlign(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let align = arg(emu, 1);
    log_red!(emu, "gdi32!SetTextAlign align:0x{:x}", align);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous alignment
    stdcall_ret(emu, 2);
}
