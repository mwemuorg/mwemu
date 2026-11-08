use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!LoadStringW id:{}", arg(emu, 1));
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
