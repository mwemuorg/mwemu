use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetExpandedNameA(emu: &mut emu::Emu) {
    get_expanded_name(emu, "GetExpandedNameA");
}

fn get_expanded_name(emu: &mut emu::Emu, api: &str) {
    let _lpszSource = WinApiAbi::arg(emu, 0) as u32;
    let _lpszBuffer = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn GetExpandedNameW(emu: &mut emu::Emu) {
    get_expanded_name(emu, "GetExpandedNameW");
}
