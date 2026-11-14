use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindExecutableA(emu: &mut emu::Emu) {
    find_executable(emu, "FindExecutableA", false);
}

pub fn FindExecutableW(emu: &mut emu::Emu) {
    find_executable(emu, "FindExecutableW", true);
}

fn find_executable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("findexecutablew");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    } else {
        let handle = helper::handler_create("findexecutablea");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    }
}
