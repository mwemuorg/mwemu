use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetGeoInfoA(emu: &mut emu::Emu) {
    get_geo_info(emu, "GetGeoInfoA");
}

fn get_geo_info(emu: &mut emu::Emu, api: &str) {
    let _Location = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _GeoType = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpGeoData = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _cchData = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _LangId = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn GetGeoInfoW(emu: &mut emu::Emu) {
    get_geo_info(emu, "GetGeoInfoW");
}
