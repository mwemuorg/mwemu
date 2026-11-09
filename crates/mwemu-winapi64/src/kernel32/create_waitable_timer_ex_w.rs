use crate::emu;
use crate::winapi::helper;

pub fn CreateWaitableTimerExW(emu: &mut emu::Emu) {
    let _security = emu.regs().rcx;
    let name = emu.regs().rdx;
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!CreateWaitableTimerExW name:'{}'", name_s);
    emu.regs_mut().rax = helper::handler_create("waitable_timer");
}
