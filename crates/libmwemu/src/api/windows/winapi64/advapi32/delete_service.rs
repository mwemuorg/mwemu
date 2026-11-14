use crate::emu;
use crate::winapi::helper;

pub fn DeleteService(emu: &mut emu::Emu) {
    let hsvc = emu.regs().rcx;
    helper::handler_close(hsvc);
    log_red!(emu, "advapi32!DeleteService 0x{:x}", hsvc);
    emu.regs_mut().rax = 1;
}
