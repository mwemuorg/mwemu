use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn LeaveCriticalSection(emu: &mut emu::Emu) {
    let crit_sect = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!LeaveCriticalSection");
    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
}
