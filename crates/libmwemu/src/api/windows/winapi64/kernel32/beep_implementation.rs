use crate::emu;
use crate::windows::constants;

pub fn BeepImplementation(emu: &mut emu::Emu) {
    let _dwFreq = emu.regs().rcx as u32;
    let _dwDuration = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!BeepImplementation");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
