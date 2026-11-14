use crate::emu;
use crate::winapi::helper;

pub fn CreateActCtxA(emu: &mut emu::Emu) {
    let p_act_ctx = emu.regs().rcx;
    log_red!(emu, "kernel32!CreateActCtxA pActCtx:0x{:x}", p_act_ctx);
    emu.regs_mut().rax = helper::handler_create("actctx://");
}

pub fn CreateActCtxW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateActCtxW");
    emu.regs_mut().rax = helper::handler_create("CreateActCtxW");
}
