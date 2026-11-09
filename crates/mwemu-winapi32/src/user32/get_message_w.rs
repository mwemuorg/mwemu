use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetMessageW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
