use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn RegisterClassA(emu: &mut emu::Emu) {
    register_class(emu, "RegisterClassA");
}

fn register_class(emu: &mut emu::Emu, api: &str) {
    let wndclass = arg(emu, 0);
    log_red!(emu, "user32!{api} 0x{:x}", wndclass);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

pub(super) fn RegisterClassW(emu: &mut emu::Emu) {
    register_class(emu, "RegisterClassW");
}
