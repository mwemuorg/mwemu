use crate::emu;
use crate::winapi::helper;

pub fn GetActiveProcessorCount(emu: &mut emu::Emu) {
    let group_number = emu.regs().rcx as u16;
    log_red!(
        emu,
        "kernel32!GetActiveProcessorCount group:{} => 4",
        group_number
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 4;
}
