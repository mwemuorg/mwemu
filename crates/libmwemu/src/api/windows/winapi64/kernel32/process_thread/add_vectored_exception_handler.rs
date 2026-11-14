use crate::emu;
pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let first = emu.regs().rcx as u32;
    let handler = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler first:{} handler:0x{:x}",
        first,
        handler
    );
    emu.add_veh(first != 0, handler);

    // the handler address doubles as the handle RemoveVectoredExceptionHandler gets
    emu.regs_mut().rax = handler;
}
