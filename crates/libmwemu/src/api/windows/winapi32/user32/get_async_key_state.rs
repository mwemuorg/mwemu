use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetAsyncKeyState(emu: &mut emu::Emu) {
    let vkey = arg(emu, 0);
    log_red!(emu, "user32!GetAsyncKeyState vKey=0x{:x}", vkey);
    emu.regs_mut().rax = 0; // not pressed
    stdcall_ret(emu, 1);
}
