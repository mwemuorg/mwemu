use crate::emu;

pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!InitializeCriticalSectionAndSpinCount cannot read crit_sect");
    let spin_count = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!InitializeCriticalSectionAndSpinCount cannot read spin_count");

    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionAndSpinCount crit_sect: 0x{:x} spin_count: {}",
        crit_sect,
        spin_count
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
