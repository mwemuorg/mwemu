use crate::emu;
use crate::winapi::helper;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let cp = emu.regs().rcx;
    log_red!(emu, "kernelbase!IsValidCodePage cp:{}", cp);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
