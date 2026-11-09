use crate::emu;

pub(super) fn crt_srand(emu: &mut emu::Emu) {
    let seed = emu.regs().rcx;
    log_red!(emu, "wincrt!srand seed:{}", seed);
    emu.set_last_error(0);
}
