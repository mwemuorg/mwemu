use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetProfileIntA(emu: &mut emu::Emu) {
    get_profile_int(emu, "GetProfileIntA", false);
}

fn get_profile_int(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = WinApiAbi::arg(emu, 0) as u32;
    let lpKeyName = WinApiAbi::arg(emu, 1) as u32;
    let _nDefault = WinApiAbi::arg(emu, 2) as u32;

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpAppName, lpKeyName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetProfileIntW(emu: &mut emu::Emu) {
    get_profile_int(emu, "GetProfileIntW", true);
}
