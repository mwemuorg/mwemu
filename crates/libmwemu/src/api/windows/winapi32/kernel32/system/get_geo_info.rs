use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetGeoInfoA(emu: &mut emu::Emu) {
    get_geo_info(emu, "GetGeoInfoA");
}

fn get_geo_info(emu: &mut emu::Emu, api: &str) {
    let _Location = WinApiAbi::arg(emu, 0) as u32;
    let _GeoType = WinApiAbi::arg(emu, 1) as u32;
    let _lpGeoData = WinApiAbi::arg(emu, 2) as u32;
    let _cchData = WinApiAbi::arg(emu, 3) as u32;
    let _LangId = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn GetGeoInfoW(emu: &mut emu::Emu) {
    get_geo_info(emu, "GetGeoInfoW");
}
