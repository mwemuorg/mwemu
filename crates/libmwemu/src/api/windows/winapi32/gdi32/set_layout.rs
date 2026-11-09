use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetLayout(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let layout = arg(emu, 1);
    log_red!(emu, "gdi32!SetLayout layout:0x{:x}", layout);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous layout
    stdcall_ret(emu, 2);
}
