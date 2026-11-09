use crate::emu;
use crate::windows::constants::errors::*;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    let pFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _bDeleteExistingResources = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let pFileName = emu.maps.read_string(pFileName as u64);

    log_red!(emu, "kernel32!BeginUpdateResourceA {}", pFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn BeginUpdateResourceW(emu: &mut emu::Emu) {
    let pFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _bDeleteExistingResources = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let pFileName = emu.maps.read_wide_string(pFileName as u64);

    log_red!(emu, "kernel32!BeginUpdateResourceW {}", pFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
