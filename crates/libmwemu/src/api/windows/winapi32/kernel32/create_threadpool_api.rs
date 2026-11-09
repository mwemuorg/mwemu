use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpool_api(emu: &mut emu::Emu) {
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateThreadpool");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpool");
}
