use crate::emu;
use crate::winapi::helper;

pub fn PFXIsPFXBlob(emu: &mut emu::Emu) {
    let blob = emu.regs().rcx;
    log_red!(emu, "crypt32!PFXIsPFXBlob blob:0x{:x}", blob);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
