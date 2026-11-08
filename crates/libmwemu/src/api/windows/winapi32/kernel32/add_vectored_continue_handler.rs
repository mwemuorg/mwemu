use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn AddVectoredContinueHandler(emu: &mut emu::Emu) {
    let first = arg(emu, 0) as u64 as u32;
    let handler = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!AddVectoredContinueHandler first:{} handler:0x{:x}",
        first,
        handler
    );
    let handle = helper::handler_create("AddVectoredContinueHandler");
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = handler;
    stdcall_ret(emu, 2);
}
