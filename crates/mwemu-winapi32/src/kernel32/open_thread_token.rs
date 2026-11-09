use crate::emu;
use crate::winapi::helper;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!OpenThreadToken");
    emu.regs_mut().rax = helper::handler_create("OpenThreadToken");
}
