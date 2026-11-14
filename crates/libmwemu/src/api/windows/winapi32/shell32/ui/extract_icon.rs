use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn ExtractIconA(emu: &mut emu::Emu) {
    extract_icon(emu, "ExtractIconA", false);
}

pub fn ExtractIconW(emu: &mut emu::Emu) {
    extract_icon(emu, "ExtractIconW", true);
}

fn extract_icon(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("extracticonw");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    } else {
        let handle = helper::handler_create("extracticona");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    }
}
