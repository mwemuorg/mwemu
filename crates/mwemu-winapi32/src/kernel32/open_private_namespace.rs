use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn OpenPrivateNamespaceA(emu: &mut emu::Emu) {
    let _lpBoundaryDescriptor = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpAliasPrefix = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!OpenPrivateNamespaceA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn OpenPrivateNamespaceW(emu: &mut emu::Emu) {
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!OpenPrivateNamespaceW");
    emu.regs_mut().rax = helper::handler_create("OpenPrivateNamespaceW");
}
