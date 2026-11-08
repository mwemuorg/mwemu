use crate::emu;
use crate::winapi::helper;

pub fn IsWindowUnicode(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "user32!IsWindowUnicode arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
