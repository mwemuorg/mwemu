use crate::emu;
use crate::winapi::helper;

pub fn CreatePrivateNamespaceA_api(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreatePrivateNamespaceA");
    emu.regs_mut().rax = helper::handler_create("CreatePrivateNamespaceA");
}
