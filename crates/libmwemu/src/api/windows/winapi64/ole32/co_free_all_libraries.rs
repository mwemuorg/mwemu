use crate::emu;
use crate::winapi::helper;

pub fn CoFreeAllLibraries(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!CoFreeAllLibraries arg1:0x{:x}", arg1);
    if arg1 != 0 {
        helper::handler_close(arg1);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
