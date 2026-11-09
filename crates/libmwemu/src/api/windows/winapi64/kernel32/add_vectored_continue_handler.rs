use crate::emu;
pub fn AddVectoredContinueHandler(emu: &mut emu::Emu) {
    let first = emu.regs().rcx as u32;
    let handler = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AddVectoredContinueHandler first:{} handler:0x{:x}",
        first,
        handler
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = handler;
}
