use crate::emu;
use crate::winapi::helper;

pub fn IsTokenUntrusted(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!IsTokenUntrusted h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    for _ in 0..2 {}
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
