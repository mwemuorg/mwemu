use crate::emu;
use crate::winapi::helper;

pub fn LockResource(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "kernelbase!LockResource 0x{:x}", arg1);
    let handle = helper::handler_create("LockResource");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
