use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    begin_update_resource(emu, "BeginUpdateResourceA", false);
}

fn begin_update_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let pFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _bDeleteExistingResources = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let pFileName = strings::read_text(emu, pFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", pFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn BeginUpdateResourceW(emu: &mut emu::Emu) {
    begin_update_resource(emu, "BeginUpdateResourceW", true);
}
