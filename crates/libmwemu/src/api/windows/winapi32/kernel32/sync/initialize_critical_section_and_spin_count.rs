use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = WinApiAbi::arg(emu, 0) as u32;
    let spin_count = WinApiAbi::arg(emu, 1) as u32;

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
