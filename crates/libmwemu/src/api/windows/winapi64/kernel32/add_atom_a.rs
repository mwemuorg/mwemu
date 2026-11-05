use crate::emu;
use crate::windows::constants;

pub fn AddAtomA(emu: &mut emu::Emu) {
    let lpString = emu.regs().rcx as u32;

    let lpString = emu.maps.read_string(lpString as u64);

    log_red!(emu, "kernel32!AddAtomA {}", lpString);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
