use crate::emu;
use crate::winapi::helper;

pub fn GetPriorityClass(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetPriorityClass h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        emu.regs_mut().rax = 0x20;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
