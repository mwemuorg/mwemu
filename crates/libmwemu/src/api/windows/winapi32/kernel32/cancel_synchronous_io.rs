use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn CancelSynchronousIo(emu: &mut emu::Emu) {
    let thread_handle = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!CancelSynchronousIo thread:0x{:x}",
        thread_handle
    );
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 1);
}
