use crate::emu;
use crate::winapi::helper;

pub fn CreateFiberEx(emu: &mut emu::Emu) {
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateFiberEx");
    emu.regs_mut().rax = helper::handler_create("CreateFiberEx");
}
