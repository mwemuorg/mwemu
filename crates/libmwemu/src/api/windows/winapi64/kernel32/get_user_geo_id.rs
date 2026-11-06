use crate::emu;

pub fn GetUserGeoID(emu: &mut emu::Emu) {
    let geo_class = emu.regs().rcx as u32;
    log_red!(emu, "kernel32!GetUserGeoID class:{} => 244 (US)", geo_class);
    let _ = emu.maps.is_mapped(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 244;
}
