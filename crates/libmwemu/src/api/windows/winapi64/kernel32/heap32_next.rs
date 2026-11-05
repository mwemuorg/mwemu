use crate::emu;
use crate::windows::constants;

pub fn Heap32Next(emu: &mut emu::Emu) {
    let _lphe = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!Heap32Next");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
