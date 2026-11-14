use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let ptr_crit_sect = WinApiAbi::arg(emu, 0) as u32;
    let spin_count = WinApiAbi::arg(emu, 1) as u32;
    let flags = WinApiAbi::arg(emu, 2) as u32;

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionEx ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.regs_mut().rax = 1;
}
