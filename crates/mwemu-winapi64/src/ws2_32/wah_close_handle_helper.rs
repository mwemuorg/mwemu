use crate::emu;
use crate::winapi::helper;

pub fn WahCloseHandleHelper(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "ws2_32!WahCloseHandleHelper handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 0;
}
