use crate::emu;
use crate::winapi::helper;
pub fn QueueUserAPC(emu: &mut emu::Emu) {
    let apc_func = emu.regs().rcx;
    let thread_handle = emu.regs().rdx;
    let data = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!QueueUserAPC func:0x{:x} thread:0x{:x} data:0x{:x}",
        apc_func,
        thread_handle,
        data
    );
    let handle = helper::handler_create("QueueUserApc");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
