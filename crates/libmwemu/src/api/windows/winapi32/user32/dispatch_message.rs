use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn DispatchMessageA(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageA");
}

fn dispatch_message(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api}");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

pub(super) fn DispatchMessageW(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageW");
}
