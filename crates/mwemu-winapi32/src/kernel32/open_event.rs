use crate::emu;
use crate::winapi::helper;

pub fn OpenEventA(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!OpenEventA");
    emu.regs_mut().rax = helper::handler_create("OpenEventA");
}

pub fn OpenEventW(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!OpenEventW");
    emu.regs_mut().rax = helper::handler_create("OpenEventW");
}
