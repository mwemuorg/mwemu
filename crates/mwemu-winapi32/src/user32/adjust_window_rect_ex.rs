use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn AdjustWindowRectEx(emu: &mut emu::Emu) {
    log_red!(emu, "user32!AdjustWindowRectEx");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
