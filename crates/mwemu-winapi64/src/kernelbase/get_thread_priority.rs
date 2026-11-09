use crate::emu;
use crate::winapi::helper;

pub fn GetThreadPriority(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetThreadPriority h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = 0x7FFF_FFFF;
    }
    emu.set_last_error(0);
}
