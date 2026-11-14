use crate::emu;
use crate::winapi::helper;

pub fn InternetLockRequestFile(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetLockRequestFile arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    if arg1 != 0 && !helper::handler_exist(arg1) {
        emu.set_last_error(12019);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
