use crate::emu;

pub fn GlobalAddAtomA(emu: &mut emu::Emu) {
    let lp_string = emu.regs().rcx as usize;
    log_red!(
        emu,
        "** {} kernel32!GlobalAddAtomA lp_string: 0x{:x}",
        emu.pos,
        lp_string
    );
    // TODO: not sure what to do
    emu.regs_mut().rax = 1;
}
