
use crate::emu;
use crate::winapi::helper;
pub fn SetTimerQueueTimer(emu: &mut emu::Emu) {
    let _TimerQueue = emu.regs().rcx as u32;
    let _Callback = emu.regs().rdx as u32;
    let _Parameter = emu.regs().r8 as u32;
    let _DueTime = emu.regs().r9 as u32;
    let _Period = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _PreferIo = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(emu, "kernel32!SetTimerQueueTimer");
    let handle = helper::handler_create("SetTimerQueueTimer");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
