use crate::emu;
use crate::winapi::helper;

pub fn LoadUrlCacheContent(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "wininet!LoadUrlCacheContent handle:0x{:x}", handle);
    let handle = helper::handler_create("LoadUrlCacheContent");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
