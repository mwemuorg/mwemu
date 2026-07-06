use crate::emu;

pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionAndSpinCount crit_sect: 0x{:x} spin_count: {}",
        crit_sect,
        spin_count
    );

    emu.regs_mut().rax = 1;
}
