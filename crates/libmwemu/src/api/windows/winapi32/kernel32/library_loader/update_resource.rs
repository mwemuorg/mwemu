use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn UpdateResourceA(emu: &mut emu::Emu) {
    update_resource(emu, "UpdateResourceA", false);
}

fn update_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hUpdate = WinApiAbi::arg(emu, 0) as u32;
    let _lpType = WinApiAbi::arg(emu, 1) as u32;
    let lpName = WinApiAbi::arg(emu, 2) as u32;
    let _wLanguage = WinApiAbi::arg(emu, 3) as u32;
    let _lpData = WinApiAbi::arg(emu, 4) as u32;
    let _cb = WinApiAbi::arg(emu, 5) as u32;

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
