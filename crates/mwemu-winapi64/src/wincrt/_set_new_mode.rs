use crate::emu;

pub(super) fn _set_new_mode(emu: &mut emu::Emu) {
    // int _set_new_mode(int newmode): selects whether malloc failures call the
    // new-handler. Returns the previous mode. We keep no CRT state, so report the
    // default (0). Pure init-time bookkeeping — a safe no-op for emulation.
    let newmode = emu.regs().rcx;
    log_red!(emu, "wincrt!_set_new_mode newmode: 0x{:x}", newmode);
    emu.regs_mut().rax = 0;
}
