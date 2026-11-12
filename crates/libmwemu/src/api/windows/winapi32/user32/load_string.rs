use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn LoadStringA(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringA");
}

fn load_string(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api} id:{}", arg(emu, 1));
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringW");
}
