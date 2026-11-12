use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let crit_sect = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!EnterCriticalSection 0x{:x}", crit_sect);
    emu.regs_mut().rax = crit_sect as u64;
    emu.stack_pop32(false);
}
