use crate::emu;
use crate::winapi::helper;

pub fn UrlZonesDetach(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!UrlZonesDetach arg1:0x{:x}", arg1);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}
