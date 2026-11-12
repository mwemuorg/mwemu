use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn InitializeCriticalSection(emu: &mut emu::Emu) {
    let ptr_crit_sect = WinApiAbi::arg(emu, 0) as u32;

    emu.stack_pop32(false);

    log_red!(
        emu,
        "kernel32!InitializeCriticalSection ptr: 0x{:x}",
        ptr_crit_sect
    );

    emu.regs_mut().rax = 1;
}
