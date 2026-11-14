use crate::emu;
use crate::winapi::helper;

pub fn BaseRegCloseKey(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!BaseRegCloseKey h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    for _ in 0..2 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
