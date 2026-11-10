use crate::emu;
use crate::winapi::helper;

pub fn CreateFile2(emu: &mut emu::Emu) {
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateFile2");
    emu.regs_mut().rax = helper::handler_create("CreateFile2");
}
