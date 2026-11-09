use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetKeyboardLayout(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetKeyboardLayout");
    emu.regs_mut().rax = 0x0409_0409;
    stdcall_ret(emu, 1);
}
