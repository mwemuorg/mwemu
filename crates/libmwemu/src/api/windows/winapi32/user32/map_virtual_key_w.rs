use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn MapVirtualKeyW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!MapVirtualKeyW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
