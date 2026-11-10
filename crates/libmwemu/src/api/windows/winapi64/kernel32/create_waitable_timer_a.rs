use crate::emu;
use crate::winapi::helper;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu, api: &str) {
    let _security = emu.regs().rcx;
    let manual_reset = emu.regs().rdx;
    let name = emu.regs().r8;
    let name_s = if name != 0 {
        emu.maps.read_string(name)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!{} manual:{} name:'{}'",
        api,
        manual_reset,
        name_s
    );
    emu.regs_mut().rax = helper::handler_create("waitable_timer");
}
