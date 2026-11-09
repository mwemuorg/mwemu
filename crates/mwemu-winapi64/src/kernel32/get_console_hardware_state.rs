use crate::emu;

pub fn GetConsoleHardwareState(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    let state_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetConsoleHardwareState h:0x{:x} flags:0x{:x} state:0x{:x}",
        h_console,
        flags_ptr,
        state_ptr
    );
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    if state_ptr != 0 {
        emu.maps.write_dword(state_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
