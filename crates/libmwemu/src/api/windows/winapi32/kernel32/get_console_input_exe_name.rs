use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetConsoleInputExeNameA(emu: &mut emu::Emu) {
    get_console_input_exe_name(emu, "GetConsoleInputExeNameA", false);
}

fn get_console_input_exe_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn GetConsoleInputExeNameW(emu: &mut emu::Emu) {
    get_console_input_exe_name(emu, "GetConsoleInputExeNameW", true);
}
