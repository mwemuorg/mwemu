use crate::emu;
use crate::windows::constants;

pub fn GetGeoInfoA(emu: &mut emu::Emu) {
    let _Location = emu.regs().rcx as u32;
    let _GeoType = emu.regs().rdx as u32;
    let _lpGeoData = emu.regs().r8 as u32;
    let _cchData = emu.regs().r9 as u32;
    let _LangId = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!GetGeoInfoA");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
