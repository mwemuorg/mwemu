use crate::emu;

pub fn SetUserGeoName(emu: &mut emu::Emu) {
    let geo_name_ptr = emu.regs().rcx;
    let name = emu.maps.read_wide_string(geo_name_ptr);
    log_red!(emu, "kernel32!SetUserGeoName '{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
