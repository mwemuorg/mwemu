use crate::emu;

pub fn GetConsoleDisplayMode(emu: &mut emu::Emu) {
    let mode_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetConsoleDisplayMode ptr:0x{:x}", mode_ptr);
    if mode_ptr != 0 {
        emu.maps.write_dword(mode_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
