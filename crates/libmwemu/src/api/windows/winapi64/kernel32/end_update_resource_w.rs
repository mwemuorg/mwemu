
use crate::windows::constants::errors::*;
use crate::emu;
use crate::winapi::helper;

pub fn EndUpdateResourceW(emu: &mut emu::Emu) {
    let _hUpdate = emu.regs().rcx as u32;
    let _fDiscard = emu.regs().rdx as u32;
    log_red!(emu, "kernel32!EndUpdateResourceW");
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
