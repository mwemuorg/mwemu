use crate::emu;
use crate::winapi::helper;

pub fn CoTestCancel(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoTestCancel");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80010002u64;
}
