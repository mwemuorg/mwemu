use crate::emu;
use crate::winapi::helper;
pub fn GlobalLock(emu: &mut emu::Emu) {
    let mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalLock mem: 0x{:x}", mem);
    let handle = helper::handler_create("GlobalLock");
    emu.set_last_error(0);
    emu.regs_mut().rax = mem;
}
