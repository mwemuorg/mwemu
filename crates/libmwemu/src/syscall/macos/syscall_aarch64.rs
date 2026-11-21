use super::dispatch;
use crate::emu::Emu;

pub fn gateway(emu: &mut Emu) {
    let number = emu.regs_aarch64().x[16];
    if !emu.call_syscall_hook(number) {
        return;
    }
    let x = &emu.regs_aarch64().x;
    dispatch(emu, number, [x[0], x[1], x[2], x[3], x[4], x[5]]);
}
