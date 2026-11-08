use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn FlushViewOfFile(emu: &mut emu::Emu) {
    let lp_base = arg(emu, 0) as u64;
    let num_bytes = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!FlushViewOfFile base:0x{:x} bytes:{}",
        lp_base,
        num_bytes
    );
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
