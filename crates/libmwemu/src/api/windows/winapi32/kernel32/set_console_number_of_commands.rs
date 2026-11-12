use crate::emu;
use crate::winapi::common::strings;

pub fn SetConsoleNumberOfCommandsA(emu: &mut emu::Emu) {
    set_console_number_of_commands(emu, "SetConsoleNumberOfCommandsA", false);
}

fn set_console_number_of_commands(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
