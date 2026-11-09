use crate::emu;

pub fn GetConsoleNlsMode(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let mode_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetConsoleNlsMode h:0x{:x} mode:0x{:x}",
        h_console,
        mode_ptr
    );
    if mode_ptr != 0 {
        emu.maps.write_dword(mode_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
