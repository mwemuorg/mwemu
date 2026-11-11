use crate::emu;

pub fn GetActiveProcessorCount(emu: &mut emu::Emu) {
    let group = emu.regs().rcx;
    let count: u64 = 4;
    log_red!(
        emu,
        "kernel32!GetActiveProcessorCount group:{} => {}",
        group,
        count
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = count;
}
