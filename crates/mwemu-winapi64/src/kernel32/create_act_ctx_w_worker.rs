use crate::emu;
use crate::winapi::helper;

pub fn CreateActCtxWWorker(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!CreateActCtxWWorker");

    emu.regs_mut().rax = helper::handler_create("CreateActCtxWWorker");
}
