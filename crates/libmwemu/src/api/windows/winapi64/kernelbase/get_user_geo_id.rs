use crate::emu;

pub fn GetUserGeoID(emu: &mut emu::Emu) {
    let geo_class = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetUserGeoID class:{}", geo_class);
    let _ = emu.maps.is_mapped(geo_class);
    emu.set_last_error(0);
    emu.regs_mut().rax = 244;
}
