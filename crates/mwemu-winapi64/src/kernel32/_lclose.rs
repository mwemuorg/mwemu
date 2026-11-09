use crate::emu;
use crate::winapi::helper;
pub fn _lclose(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!_lclose handle:0x{:x}", handle);
    if helper::handler_exist(handle) {
        helper::handler_close(handle);
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = -1i64 as u64;
    }
}
