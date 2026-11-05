use crate::emu;

pub fn LocalCompact(emu: &mut emu::Emu) {
    let min_free = emu.regs().rcx;
    log_red!(emu, "kernel32!LocalCompact minFree:{}", min_free);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00FF_FFFF;
}
