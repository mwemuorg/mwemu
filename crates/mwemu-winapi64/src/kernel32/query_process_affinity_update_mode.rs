use crate::emu;

pub fn QueryProcessAffinityUpdateMode(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let lpdw_flags = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!QueryProcessAffinityUpdateMode hProcess:0x{:x} pFlags:0x{:x}",
        h_process,
        lpdw_flags
    );
    if lpdw_flags != 0 {
        emu.maps.write_dword(lpdw_flags, 0);
    }
    emu.regs_mut().rax = 1;
}
