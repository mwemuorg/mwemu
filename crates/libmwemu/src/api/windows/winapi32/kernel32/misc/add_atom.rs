use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn AddAtomA(emu: &mut emu::Emu) {
    add_atom(emu, "AddAtomA", false);
}

fn add_atom(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpString = WinApiAbi::arg(emu, 0) as u32;

    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn AddAtomW(emu: &mut emu::Emu) {
    add_atom(emu, "AddAtomW", true);
}
