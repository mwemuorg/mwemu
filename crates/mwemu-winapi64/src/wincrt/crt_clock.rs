use crate::emu;

pub(super) fn crt_clock(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!clock => 1000");
    emu.regs_mut().rax = 1000;
}
