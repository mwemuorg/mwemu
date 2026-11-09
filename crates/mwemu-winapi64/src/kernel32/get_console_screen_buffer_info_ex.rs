use crate::emu;

pub fn GetConsoleScreenBufferInfoEx(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let info_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetConsoleScreenBufferInfoEx h:0x{:x} ptr:0x{:x}",
        h_console,
        info_ptr
    );
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 96);
        emu.maps.write_word(info_ptr + 4, 80);
        emu.maps.write_word(info_ptr + 6, 25);
        emu.maps.write_word(info_ptr + 8, 0);
        emu.maps.write_word(info_ptr + 10, 0);
        emu.maps.write_word(info_ptr + 12, 7);
        emu.maps.write_word(info_ptr + 14, 0);
        emu.maps.write_word(info_ptr + 16, 0);
        emu.maps.write_word(info_ptr + 18, 79);
        emu.maps.write_word(info_ptr + 20, 24);
        emu.maps.write_word(info_ptr + 22, 80);
        emu.maps.write_word(info_ptr + 24, 25);
    }
    emu.regs_mut().rax = 1;
}
