use crate::emu;

pub fn GlobalCompact(emu: &mut emu::Emu) {
    let min_free = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalCompact minFree:{}", min_free);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xFFFF_FFFF;
}
