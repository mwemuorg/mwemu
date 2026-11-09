
use crate::windows::constants::errors::*;
use crate::emu;

pub fn WaitNamedPipeA(emu: &mut emu::Emu) {
    let lpNamedPipeName = emu.regs().rcx as u32;
    let _nTimeOut = emu.regs().rdx as u32;

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!WaitNamedPipeA {}", lpNamedPipeName);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
