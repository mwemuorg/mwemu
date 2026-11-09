use crate::emu;
use crate::winapi::helper;

pub fn CreateJobObjectA_api(emu: &mut emu::Emu) {
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateJobObjectA");
    emu.regs_mut().rax = helper::handler_create("CreateJobObjectA");
}
