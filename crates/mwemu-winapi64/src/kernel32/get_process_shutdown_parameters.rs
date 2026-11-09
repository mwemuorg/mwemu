use crate::emu;

pub fn GetProcessShutdownParameters(emu: &mut emu::Emu) {
    let level_ptr = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetProcessShutdownParameters level:0x{:x} flags:0x{:x}",
        level_ptr,
        flags_ptr
    );
    if level_ptr != 0 {
        emu.maps.write_dword(level_ptr, 0x280);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
