use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn AddRefActCtx(emu: &mut emu::Emu) {
    let h_act_ctx = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!AddRefActCtx hActCtx:0x{:x}", h_act_ctx);
    let handle = helper::handler_create("AddRefActCtx");
    emu.set_last_error(0);
    stdcall_ret(emu, 1);
}
