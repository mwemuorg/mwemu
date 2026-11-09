use crate::emu;

pub fn SafeArrayGetUBound(emu: &mut emu::Emu) {
    let psa = emu.regs().rcx;
    let ndim = emu.regs().rdx as u32;
    let pubound = emu.regs().r8;
    log_red!(
        emu,
        "oleaut32!SafeArrayGetUBound psa: 0x{:x} dim: {}",
        psa,
        ndim
    );
    if psa != 0 && pubound != 0 {
        let count = emu.maps.read_dword(psa + 24).unwrap_or(0);
        let lbound = emu.maps.read_dword(psa + 28).unwrap_or(0);
        let ubound = if count > 0 {
            lbound + count - 1
        } else {
            lbound
        };
        emu.maps.write_dword(pubound, ubound);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
