use crate::emu;
use crate::winapi::helper;

pub fn CoSetState(emu: &mut emu::Emu) {
    let punk = emu.regs().rcx;
    log_red!(emu, "ole32!CoSetState punk:0x{:x}", punk);
    if punk != 0 {
        let _ = helper::handler_create("CoState");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
