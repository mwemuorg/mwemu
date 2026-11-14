use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerA", false);
}

pub fn CreateWaitableTimerW(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerW", true);
}

fn create_waitable_timer(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _security = WinApiAbi::arg(emu, 0);
    let manual_reset = WinApiAbi::arg(emu, 1);
    let name = WinApiAbi::arg(emu, 2);
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
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
