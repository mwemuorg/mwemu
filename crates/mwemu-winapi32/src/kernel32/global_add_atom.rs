use crate::emu;
use crate::windows::constants::errors::*;

pub fn GlobalAddAtomA(emu: &mut emu::Emu) {
    let lp_string = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let atom_name = emu.maps.read_string(lp_string);

    log_red!(emu, "kernel32!GlobalAddAtomA name: `{}`", atom_name);

    emu.stack_pop32(false);

    // TODO: implement real atom table
    emu.regs_mut().rax = 0xC001; // Return a dummy atom
}

pub fn GlobalAddAtomW(emu: &mut emu::Emu) {
    let lpString = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomW {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
