use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateHardLinkA(emu: &mut emu::Emu) {
    create_hard_link(emu, "CreateHardLinkA", false);
}

fn create_hard_link(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}

pub fn CreateHardLinkW(emu: &mut emu::Emu) {
    create_hard_link(emu, "CreateHardLinkW", true);
}
