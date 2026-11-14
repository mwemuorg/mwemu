use crate::emu;
use crate::winapi::helper;

pub fn CreatePrivateNamespaceA(emu: &mut emu::Emu) {
    create_private_namespace(emu, "CreatePrivateNamespaceA");
}

fn create_private_namespace(emu: &mut emu::Emu, api: &str) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreatePrivateNamespaceW(emu: &mut emu::Emu) {
    create_private_namespace(emu, "CreatePrivateNamespaceW");
}
