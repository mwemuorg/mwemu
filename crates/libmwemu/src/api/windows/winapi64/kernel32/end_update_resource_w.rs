use crate::emu;
use crate::windows::constants;

pub fn EndUpdateResourceW(emu: &mut emu::Emu) {
    let _hUpdate = emu.regs().rcx as u32;
    let _fDiscard = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!EndUpdateResourceW");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
