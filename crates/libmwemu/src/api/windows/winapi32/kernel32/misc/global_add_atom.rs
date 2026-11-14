use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GlobalAddAtomA(emu: &mut emu::Emu) {
    let lp_string = WinApiAbi::arg(emu, 0);

    let atom_name = emu.maps.read_string(lp_string);

    log_red!(emu, "kernel32!GlobalAddAtomA name: `{}`", atom_name);

    emu.stack_pop32(false);

    // TODO: implement real atom table
    emu.regs_mut().rax = 0xC001; // Return a dummy atom
}

pub fn GlobalAddAtomW(emu: &mut emu::Emu) {
    let lpString = WinApiAbi::arg(emu, 0) as u32;

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomW {}", lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
