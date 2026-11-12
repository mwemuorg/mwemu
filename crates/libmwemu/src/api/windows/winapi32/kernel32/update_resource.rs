use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn UpdateResourceA(emu: &mut emu::Emu) {
    update_resource(emu, "UpdateResourceA", false);
}

fn update_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hUpdate = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpType = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _wLanguage = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpData = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _cb = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn UpdateResourceW(emu: &mut emu::Emu) {
    update_resource(emu, "UpdateResourceW", true);
}
