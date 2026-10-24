use crate::emu;

pub fn LeaveCriticalSection(emu: &mut emu::Emu) {
    let crit_sect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!LeaveCriticalSection");
    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
}
