use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GlobalAddAtomExA(emu: &mut emu::Emu) {
    global_add_atom_ex(emu, "GlobalAddAtomExA", false);
}

fn global_add_atom_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpString = WinApiAbi::arg(emu, 0) as u32;
    let _Flags = WinApiAbi::arg(emu, 1) as u32;

    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn GlobalAddAtomExW(emu: &mut emu::Emu) {
    global_add_atom_ex(emu, "GlobalAddAtomExW", true);
}
