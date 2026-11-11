use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    for _ in 0..11u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateProcessAsUserA");
    emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserA");
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    for _ in 0..11u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateProcessAsUserW");
    emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserW");
}
