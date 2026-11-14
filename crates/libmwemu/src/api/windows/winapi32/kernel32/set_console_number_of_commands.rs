use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SetConsoleNumberOfCommandsA(emu: &mut emu::Emu) {
    set_console_number_of_commands(emu, "SetConsoleNumberOfCommandsA", false);
}

fn set_console_number_of_commands(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn SetConsoleNumberOfCommandsW(emu: &mut emu::Emu) {
    set_console_number_of_commands(emu, "SetConsoleNumberOfCommandsW", true);
}
