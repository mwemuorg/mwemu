use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreExW(emu: &mut emu::Emu) {
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateSemaphoreExW");
    emu.regs_mut().rax = helper::handler_create("CreateSemaphoreExW");
}
