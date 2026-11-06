use crate::emu;

pub fn EnumSystemGeoID(emu: &mut emu::Emu) {
    let geoClass = emu.regs().rcx;
    log_red!(emu, "kernel32!EnumSystemGeoID geoClass:0x{:x}", geoClass);
    let _ = emu.maps.is_mapped(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
