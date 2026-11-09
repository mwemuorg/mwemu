use crate::emu;

pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let spin_count = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

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
