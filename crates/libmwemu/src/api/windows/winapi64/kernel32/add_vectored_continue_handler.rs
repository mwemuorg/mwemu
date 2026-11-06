
use crate::emu;
use crate::winapi::helper;
pub fn AddVectoredContinueHandler(emu: &mut emu::Emu) {
    let first = emu.regs().rcx as u32;
    let handler = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AddVectoredContinueHandler first:{} handler:0x{:x}",
        first,
        handler
    );
    let handle = helper::handler_create("AddVectoredContinueHandler");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = handler;
}
