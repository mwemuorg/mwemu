use crate::emu;
use crate::winapi::helper;

pub fn CreateTimerQueueTimer(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateTimerQueueTimer");
    log_red!(emu, "kernelbase!CreateTimerQueueTimer => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
