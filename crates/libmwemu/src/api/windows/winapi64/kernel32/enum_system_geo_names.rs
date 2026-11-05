use crate::emu;

pub fn EnumSystemGeoNames(emu: &mut emu::Emu) {
    let geoClass = emu.regs().rcx;
    log_red!(emu, "kernel32!EnumSystemGeoNames geoClass:0x{:x}", geoClass);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
