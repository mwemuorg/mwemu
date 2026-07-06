use crate::emu;

pub fn InitOnceBeginInitialize(emu: &mut emu::Emu) {
    let lp_init_once = emu.regs().rcx as usize;
    let dw_flags = emu.regs().rdx as usize;
    let f_pending = emu.regs().r8 as usize;
    let lp_context = emu.regs().r9 as usize;
    log_red!(
        emu,
        "** {} kernel32!InitOnceBeginInitialize {:x} {:x} {:x} {:x}",
        emu.pos,
        lp_init_once,
        dw_flags,
        f_pending,
        lp_context
    );
    // TODO: implement this
    emu.regs_mut().rax = 1;
}
