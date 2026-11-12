use crate::emu;
use crate::winapi::helper;

pub fn CreateEventExA(emu: &mut emu::Emu) {
    create_event_ex(emu, "CreateEventExA");
}

fn create_event_ex(emu: &mut emu::Emu, api: &str) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateEventExW(emu: &mut emu::Emu) {
    create_event_ex(emu, "CreateEventExW");
}
