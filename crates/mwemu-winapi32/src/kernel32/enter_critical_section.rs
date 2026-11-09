use crate::emu;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let crit_sect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!EnterCriticalSection 0x{:x}", crit_sect);
    emu.regs_mut().rax = crit_sect as u64;
    emu.stack_pop32(false);
}
