use crate::emu;
use crate::winapi::helper;

pub fn CoDecrementMTAUsage(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!CoDecrementMTAUsage arg1:0x{:x}", arg1);
    if helper::handler_exist(emu.regs().rcx) {
        helper::handler_close(emu.regs().rcx);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
