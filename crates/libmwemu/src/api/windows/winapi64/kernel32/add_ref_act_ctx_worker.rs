use crate::emu;

pub fn AddRefActCtxWorker(emu: &mut emu::Emu) {
    let h_act_ctx = emu.regs().rcx;
    log_red!(emu, "kernel32!AddRefActCtxWorker hActCtx:0x{:x}", h_act_ctx);
    emu.set_last_error(0);
}
