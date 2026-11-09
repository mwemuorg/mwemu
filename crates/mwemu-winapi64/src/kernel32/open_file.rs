use crate::emu;
use crate::winapi::helper;

pub fn OpenFile(emu: &mut emu::Emu) {
    let _lpFileName = emu.regs().rcx as u32;
    let _lpReOpenBuff = emu.regs().rdx as u32;
    let _uStyle = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!OpenFile");

    emu.regs_mut().rax = helper::handler_create("OpenFile");
}
