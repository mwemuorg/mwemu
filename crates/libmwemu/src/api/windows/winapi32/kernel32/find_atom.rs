use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn FindAtomA(emu: &mut emu::Emu) {
    find_atom(emu, "FindAtomA", false);
}

fn find_atom(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpString = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn FindAtomW(emu: &mut emu::Emu) {
    find_atom(emu, "FindAtomW", true);
}
