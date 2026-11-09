use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexExA(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateMutexExA");
    emu.regs_mut().rax = helper::handler_create("CreateMutexExA");
}

pub fn CreateMutexExW(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateMutexExW");
    emu.regs_mut().rax = helper::handler_create("CreateMutexExW");
}
