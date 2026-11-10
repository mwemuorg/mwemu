
use crate::windows::constants::errors::*;
use crate::emu;

pub fn Heap32ListFirst(emu: &mut emu::Emu) {
    let _hSnapshot = emu.regs().rcx as u32;
    let _lphl = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!Heap32ListFirst");

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
