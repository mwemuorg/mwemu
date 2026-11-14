use crate::emu;
use crate::winapi::helper;

pub fn GlobalLock(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "kernelbase!GlobalLock 0x{:x}", arg1);
    let handle = helper::handler_create("GlobalLock");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
