use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    begin_update_resource(emu, "BeginUpdateResourceA", false);
}

fn begin_update_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let pFileName = WinApiAbi::arg(emu, 0) as u32;
    let _bDeleteExistingResources = WinApiAbi::arg(emu, 1) as u32;

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
