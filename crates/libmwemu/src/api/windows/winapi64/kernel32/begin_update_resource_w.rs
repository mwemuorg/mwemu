use crate::emu;
use crate::windows::constants;

pub fn BeginUpdateResourceW(emu: &mut emu::Emu) {
    let pFileName = emu.regs().rcx as u32;
    let _bDeleteExistingResources = emu.regs().rdx as u32;

    let pFileName = emu.maps.read_wide_string(pFileName as u64);

    log_red!(emu, "kernel32!BeginUpdateResourceW {}", pFileName);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
