use crate::emu;
use crate::windows::constants::errors::*;

pub fn SetFileShortNameA(emu: &mut emu::Emu) {
    let _hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpShortName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpShortName = emu.maps.read_string(lpShortName as u64);

    log_red!(emu, "kernel32!SetFileShortNameA {}", lpShortName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn SetFileShortNameW(emu: &mut emu::Emu) {
    let _hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpShortName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpShortName = emu.maps.read_wide_string(lpShortName as u64);

    log_red!(emu, "kernel32!SetFileShortNameW {}", lpShortName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
