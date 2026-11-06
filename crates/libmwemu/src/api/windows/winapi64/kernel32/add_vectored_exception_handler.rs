use crate::emu;
use crate::winapi::helper;
pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = emu.regs().rcx as u32;
    let handler = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler first:{} handler:0x{:x}",
        first,
        handler
    );
    let handle = helper::handler_create("AddVectoredExceptionHandler");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.add_veh(first != 0, handler);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x2c2878;
}
