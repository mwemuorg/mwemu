use crate::emu;
pub fn CancelTimerQueueTimer(emu: &mut emu::Emu) {
    let _TimerQueue = emu.regs().rcx as u32;
    let _Timer = emu.regs().rdx as u32;
    log_red!(emu, "kernel32!CancelTimerQueueTimer");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
