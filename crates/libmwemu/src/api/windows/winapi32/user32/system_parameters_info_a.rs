use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SystemParametersInfoA(emu: &mut emu::Emu) {
    let action = arg(emu, 0);
    log_red!(emu, "user32!SystemParametersInfoA uiAction: 0x{:x}", action);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
