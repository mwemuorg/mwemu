use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn FlushInstructionCache(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let lp_base = arg(emu, 1) as u64;
    let size = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!FlushInstructionCache hProcess:0x{:x} base:0x{:x} size:{}",
        h_process,
        lp_base,
        size
    );
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
