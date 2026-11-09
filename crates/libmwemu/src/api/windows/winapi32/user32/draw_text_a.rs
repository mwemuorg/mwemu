use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn DrawTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(arg(emu, 1) as u64);
    log_red!(emu, "user32!DrawTextA '{}'", text);
    emu.regs_mut().rax = 20;
    stdcall_ret(emu, 5);
}
