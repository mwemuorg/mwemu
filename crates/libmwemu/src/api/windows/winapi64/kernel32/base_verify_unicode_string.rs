use crate::emu;
use crate::winapi::helper;
pub fn BaseVerifyUnicodeString(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!BaseVerifyUnicodeString param1:0x{:x}",
        param1
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
