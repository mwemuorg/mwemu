use crate::emu;
use crate::winapi::helper;

pub fn AcquireSRWLockExclusive(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "kernelbase!AcquireSRWLockExclusive 0x{:x}", arg1);
    let handle = helper::handler_create("AcquireSRWLockExclusive");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
