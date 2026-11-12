use crate::emu;
use crate::windows::constants::errors::*;

pub fn GlobalGetAtomNameA(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameA");
}

fn global_get_atom_name(emu: &mut emu::Emu, api: &str) {
    let _nAtom = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpBuffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GlobalGetAtomNameW(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameW");
}
