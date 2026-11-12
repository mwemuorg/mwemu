use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetConsoleKeyboardLayoutNameA(emu: &mut emu::Emu) {
    get_console_keyboard_layout_name(emu, "GetConsoleKeyboardLayoutNameA");
}

fn get_console_keyboard_layout_name(emu: &mut emu::Emu, api: &str) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn GetConsoleKeyboardLayoutNameW(emu: &mut emu::Emu) {
    get_console_keyboard_layout_name(emu, "GetConsoleKeyboardLayoutNameW");
}
