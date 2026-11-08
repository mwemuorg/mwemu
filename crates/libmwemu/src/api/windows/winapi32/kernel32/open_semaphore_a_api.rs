use crate::emu;
use crate::winapi::helper;

pub fn OpenSemaphoreA_api(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!OpenSemaphoreA");
    emu.regs_mut().rax = helper::handler_create("OpenSemaphoreA");
}
