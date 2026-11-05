use crate::emu;

pub fn GetMaximumProcessorCount(emu: &mut emu::Emu) {
    let group_number = emu.regs().rcx as u16;
    log_red!(
        emu,
        "kernel32!GetMaximumProcessorCount group:{} => 4",
        group_number
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 4;
}
