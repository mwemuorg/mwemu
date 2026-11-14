use crate::emu;

pub(super) fn signal(emu: &mut emu::Emu) {
    let sig = emu.regs().rcx;
    let handler = emu.regs().rdx;
    log_red!(emu, "wincrt!signal sig:{} handler:0x{:x}", sig, handler);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // SIG_DFL
}
