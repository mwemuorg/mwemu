use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileExA_api(emu: &mut emu::Emu) {
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!FindFirstFileExA");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileExA");
}
