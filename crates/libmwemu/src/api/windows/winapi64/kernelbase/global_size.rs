use crate::emu;
use crate::winapi::helper;

pub fn GlobalSize(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernelbase!GlobalSize h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        emu.regs_mut().rax = 4096;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
