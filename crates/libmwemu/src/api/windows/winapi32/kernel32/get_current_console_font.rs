use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCurrentConsoleFont(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let max_window = arg(emu, 1);
    let font_info_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetCurrentConsoleFont h:0x{:x} max:{} ptr:0x{:x}",
        h_console,
        max_window,
        font_info_ptr
    );
    if font_info_ptr != 0 {
        emu.maps.write_dword(font_info_ptr, 0);
        emu.maps.write_word(font_info_ptr + 4, 8);
        emu.maps.write_word(font_info_ptr + 6, 16);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
