use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn QueueUserAPC(emu: &mut emu::Emu) {
    let apc_func = arg(emu, 0) as u64;
    let thread_handle = arg(emu, 1) as u64;
    let data = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!QueueUserAPC func:0x{:x} thread:0x{:x} data:0x{:x}",
        apc_func,
        thread_handle,
        data
    );
    let handle = helper::handler_create("QueueUserApc");
    let out_ptr = arg(emu, 2) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
