use crate::emu;

pub fn SafeArrayGetLBound(emu: &mut emu::Emu) {
    let psa = emu.regs().rcx;
    let ndim = emu.regs().rdx as u32;
    let plbound = emu.regs().r8;
    log_red!(
        emu,
        "oleaut32!SafeArrayGetLBound psa: 0x{:x} dim: {}",
        psa,
        ndim
    );
    if psa != 0 && plbound != 0 {
        let lbound = emu.maps.read_dword(psa + 28).unwrap_or(0);
        emu.maps.write_dword(plbound, lbound);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
