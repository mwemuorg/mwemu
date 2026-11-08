use crate::emu;
use crate::winapi::helper;
pub fn AddRefActCtx(emu: &mut emu::Emu) {
    let h_act_ctx = emu.regs().rcx;
    log_red!(emu, "kernel32!AddRefActCtx hActCtx:0x{:x}", h_act_ctx);
    let handle = helper::handler_create("AddRefActCtx");
    emu.set_last_error(0);
}
