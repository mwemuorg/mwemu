use crate::emu;

pub fn GetConsoleCursorInfo(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let cursor_info_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetConsoleCursorInfo h:0x{:x} ptr:0x{:x}",
        h_console,
        cursor_info_ptr
    );
    if cursor_info_ptr != 0 {
        emu.maps.write_dword(cursor_info_ptr, 25);
        emu.maps.write_dword(cursor_info_ptr + 4, 1);
    }
    emu.regs_mut().rax = 1;
}
