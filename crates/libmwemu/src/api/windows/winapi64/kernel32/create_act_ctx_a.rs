use crate::emu;

pub fn CreateActCtxA(emu: &mut emu::Emu) {
    let p_act_ctx = emu.regs().rcx as usize;
    log_red!(emu, "** {} kernel32!CreateActCtxA {:x}", emu.pos, p_act_ctx);
    // TODO: implement this
    emu.regs_mut().rax = 1;
}
