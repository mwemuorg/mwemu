use crate::emu;

pub fn QueryThreadProfiling(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let p_enabled = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!QueryThreadProfiling hThread:0x{:x} pEnabled:0x{:x}",
        h_thread,
        p_enabled
    );
    if p_enabled != 0 {
        emu.maps.write_dword(p_enabled, 0);
    }
    emu.regs_mut().rax = 0;
}
