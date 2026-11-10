use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn CreateActCtxA(emu: &mut emu::Emu) {
    let _pActCtx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!CreateActCtxA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn CreateActCtxW(emu: &mut emu::Emu) {
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateActCtxW");
    emu.regs_mut().rax = helper::handler_create("CreateActCtxW");
}
