use crate::emu;

pub fn GetConsoleCursorMode(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let mode_ptr = emu.regs().rdx;
    let visible_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetConsoleCursorMode h:0x{:x} mode:0x{:x} vis:0x{:x}",
        h_console,
        mode_ptr,
        visible_ptr
    );
    if mode_ptr != 0 {
        emu.maps.write_dword(mode_ptr, 0);
    }
    if visible_ptr != 0 {
        emu.maps.write_dword(visible_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}
