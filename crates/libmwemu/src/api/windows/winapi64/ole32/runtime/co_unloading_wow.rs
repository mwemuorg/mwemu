use crate::emu;
use crate::winapi::helper;

pub fn CoUnloadingWOW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!CoUnloadingWOW arg1:0x{:x}", arg1);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {}
}
