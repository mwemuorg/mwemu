use crate::emu;

pub fn AreShortNamesEnabled(emu: &mut emu::Emu) {
    let h_volume = emu.regs().rcx;
    let p_enabled = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AreShortNamesEnabled hVolume:0x{:x} pEnabled:0x{:x}",
        h_volume,
        p_enabled
    );
    if p_enabled != 0 {
        emu.maps.write_dword(p_enabled, 0);
    }
    emu.regs_mut().rax = 1;
}
