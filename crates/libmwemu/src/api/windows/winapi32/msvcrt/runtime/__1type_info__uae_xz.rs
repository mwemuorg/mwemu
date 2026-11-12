use crate::emu;

pub fn __1type_info__UAE_XZ(emu: &mut emu::Emu) {
    let this = emu.regs().get_ecx();
    log_red!(emu, "msvcrt!type_info::~type_info this:0x{:x}", this);
    if this != 0 && emu.maps.is_mapped(this) {
        let _vt = emu.maps.read_dword(this).unwrap_or(0);
    }
    emu.set_last_error(0);
}
