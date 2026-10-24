use crate::emu;
use crate::windows::constants;

pub fn GetGeoInfoW(emu: &mut emu::Emu) {
    let _Location = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _GeoType = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpGeoData = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _cchData = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _LangId = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!GetGeoInfoW");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
