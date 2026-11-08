use crate::emu;
use crate::winapi::helper;
pub fn CancelTimerQueueTimer(emu: &mut emu::Emu) {
    let _TimerQueue = emu.regs().rcx as u32;
    let _Timer = emu.regs().rdx as u32;
    log_red!(emu, "kernel32!CancelTimerQueueTimer");
    let handle = helper::handler_create("CancelTimerQueueTimer");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
