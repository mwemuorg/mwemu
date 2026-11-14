use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn BeginPaint(emu: &mut emu::Emu) {
    log_red!(emu, "user32!BeginPaint");
    emu.regs_mut().rax = 0x1337_0002;
    stdcall_ret(emu, 2);
}
