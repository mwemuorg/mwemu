use crate::emu;

pub fn GetCurrentConsoleFont(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let max_window = emu.regs().rdx as u32;
    let font_info_ptr = emu.regs().r8;
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
}
