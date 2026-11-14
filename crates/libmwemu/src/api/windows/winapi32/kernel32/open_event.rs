use crate::emu;
use crate::winapi::helper;

pub fn OpenEventA(emu: &mut emu::Emu) {
    open_event(emu, "OpenEventA");
}

fn open_event(emu: &mut emu::Emu, api: &str) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn OpenEventW(emu: &mut emu::Emu) {
    open_event(emu, "OpenEventW");
}
