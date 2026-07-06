use crate::{emu, windows::constants};

pub fn Thread32Next(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let entry = emu.regs().rdx;

    log_red!(emu, "kernel32!Thread32Next");

    emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}
