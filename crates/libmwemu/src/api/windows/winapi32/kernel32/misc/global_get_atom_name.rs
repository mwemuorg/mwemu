use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GlobalGetAtomNameA(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameA");
}

fn global_get_atom_name(emu: &mut emu::Emu, api: &str) {
    let _nAtom = WinApiAbi::arg(emu, 0) as u32;
    let _lpBuffer = WinApiAbi::arg(emu, 1) as u32;
    let _nSize = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GlobalGetAtomNameW(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameW");
}
