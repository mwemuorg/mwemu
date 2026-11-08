use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreW(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateSemaphoreW");
    emu.regs_mut().rax = helper::handler_create("CreateSemaphoreW");
}
