use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SetWindowTextW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(arg(emu, 1) as u64);
    log_red!(emu, "user32!SetWindowTextW '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
