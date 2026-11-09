
use crate::windows::constants::errors::*;
use crate::emu;

pub fn ActivateActCtxWorker(emu: &mut emu::Emu) {
    let _Handle = emu.regs().rcx as u32;
    let _Cookie = emu.regs().rdx as u32;
    log_red!(emu, "kernel32!ActivateActCtxWorker");
    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
