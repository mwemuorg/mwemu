use crate::emu;
use crate::winapi::helper;

pub fn CreateMemoryResourceNotification_api(emu: &mut emu::Emu) {
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateMemoryResourceNotification");
    emu.regs_mut().rax = helper::handler_create("CreateMemoryResourceNotification");
}
