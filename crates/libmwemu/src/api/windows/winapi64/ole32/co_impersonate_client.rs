use crate::emu;
use crate::winapi::helper;

pub fn CoImpersonateClient(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoImpersonateClient");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
