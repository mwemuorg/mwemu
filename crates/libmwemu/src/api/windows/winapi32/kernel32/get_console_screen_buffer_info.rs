use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleScreenBufferInfo(emu: &mut emu::Emu) {
    let _h_console = arg(emu, 0) as u64;
    let info_ptr = arg(emu, 1) as u64;

    log_red!(emu, "kernel32!GetConsoleScreenBufferInfo");

    if info_ptr != 0 {
        // CONSOLE_SCREEN_BUFFER_INFO
        // dwSize (COORD: 2 x SHORT) at offset 0
        emu.maps.write_word(info_ptr, 120); // width
        emu.maps.write_word(info_ptr + 2, 9001); // height (scroll buffer)
        // dwCursorPosition (COORD) at offset 4
        emu.maps.write_word(info_ptr + 4, 0); // X
        emu.maps.write_word(info_ptr + 6, 0); // Y
        // wAttributes (WORD) at offset 8
        emu.maps.write_word(info_ptr + 8, 7); // default attrs
        // srWindow (SMALL_RECT: 4 x SHORT) at offset 10
        emu.maps.write_word(info_ptr + 10, 0); // Left
        emu.maps.write_word(info_ptr + 12, 0); // Top
        emu.maps.write_word(info_ptr + 14, 119); // Right
        emu.maps.write_word(info_ptr + 16, 29); // Bottom
        // dwMaximumWindowSize (COORD) at offset 18
        emu.maps.write_word(info_ptr + 18, 120);
        emu.maps.write_word(info_ptr + 20, 30);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
