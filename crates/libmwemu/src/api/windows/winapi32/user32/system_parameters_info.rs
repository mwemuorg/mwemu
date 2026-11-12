use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SystemParametersInfoA(emu: &mut emu::Emu) {
    system_parameters_info(emu, "SystemParametersInfoA", false);
}

pub fn SystemParametersInfoW(emu: &mut emu::Emu) {
    system_parameters_info(emu, "SystemParametersInfoW", true);
}

fn system_parameters_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
        let a0 = arg(emu, 0) as u64;
        let a1 = arg(emu, 1) as u64;
        log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
        emu.set_last_error(0);
    } else {
        let action = arg(emu, 0);
        log_red!(emu, "user32!{api} uiAction: 0x{:x}", action);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
