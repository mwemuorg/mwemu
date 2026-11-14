use crate::emu;

pub(super) fn crt_lock_noop(emu: &mut emu::Emu, api: &str) {
    let lock_num = emu.regs().rcx;
    log_red!(emu, "wincrt!{} lock:{}", api, lock_num);
    emu.set_last_error(0);
}
