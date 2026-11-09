use crate::emu;

pub fn GetCurrentConsoleFontEx(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let max_window = emu.regs().rdx as u32;
    let font_info_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetCurrentConsoleFontEx h:0x{:x} max:{} ptr:0x{:x}",
        h_console,
        max_window,
        font_info_ptr
    );
    if font_info_ptr != 0 {
        emu.maps.write_dword(font_info_ptr, 84);
        emu.maps.write_dword(font_info_ptr + 4, 0);
        emu.maps.write_word(font_info_ptr + 8, 8);
        emu.maps.write_word(font_info_ptr + 10, 16);
        emu.maps.write_dword(font_info_ptr + 12, 0);
        emu.maps.write_dword(font_info_ptr + 16, 400);
    }
    emu.regs_mut().rax = 1;
}
