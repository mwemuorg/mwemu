use crate::emu;
use crate::winapi::helper;

pub fn CreateDirectoryExA(emu: &mut emu::Emu) {
    let _lpTemplateDirectory = emu.regs().rcx as u32;
    let _lpNewDirectory = emu.regs().rdx as u32;
    let _lpSecurityAttributes = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!CreateDirectoryExA");

    emu.regs_mut().rax = helper::handler_create("CreateDirectoryExA");
}
