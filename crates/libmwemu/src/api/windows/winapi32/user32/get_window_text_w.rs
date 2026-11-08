use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetWindowTextW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowTextW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
