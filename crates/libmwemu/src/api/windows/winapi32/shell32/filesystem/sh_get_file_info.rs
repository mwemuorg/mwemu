use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHGetFileInfo(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "shell32!SHGetFileInfo h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHGetFileInfoA(emu: &mut emu::Emu) {
    sh_get_file_info(emu, "SHGetFileInfoA", false);
}

pub fn SHGetFileInfoW(emu: &mut emu::Emu) {
    sh_get_file_info(emu, "SHGetFileInfoW", true);
}

fn sh_get_file_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("shgetfileinfow");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 5);
    } else {
        let handle = helper::handler_create("shgetfileinfoa");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 5);
    }
}
