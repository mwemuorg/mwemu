use crate::emu;
use crate::winapi::helper;

pub fn CreateDirectoryTransactedA(emu: &mut emu::Emu) {
    let _lpTemplateDirectory = emu.regs().rcx as u32;
    let _lpNewDirectory = emu.regs().rdx as u32;
    let _lpSecurityAttributes = emu.regs().r8 as u32;
    let _hTransaction = emu.regs().r9 as u32;

    log_red!(emu, "kernel32!CreateDirectoryTransactedA");

    emu.regs_mut().rax = helper::handler_create("CreateDirectoryTransactedA");
}
