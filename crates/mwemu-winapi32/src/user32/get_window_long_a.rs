use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetWindowLongA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowLongA idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
