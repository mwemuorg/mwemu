use crate::emu;
use crate::winapi::helper;

pub fn CreateEventExA(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateEventExA");
    emu.regs_mut().rax = helper::handler_create("CreateEventExA");
}

pub fn CreateEventExW(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateEventExW");
    emu.regs_mut().rax = helper::handler_create("CreateEventExW");
}
