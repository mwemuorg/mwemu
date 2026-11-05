use crate::emu;
use crate::winapi::helper;

pub fn OpenPrivateNamespaceA(emu: &mut emu::Emu) {
    let _lpBoundaryDescriptor = emu.regs().rcx as u32;
    let _lpAliasPrefix = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!OpenPrivateNamespaceA");

    emu.regs_mut().rax = helper::handler_create("OpenPrivateNamespaceA");
}
