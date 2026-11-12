use crate::emu;
use crate::winapi::helper;

pub fn CreateMailslotA(emu: &mut emu::Emu) {
    create_mailslot(emu, "CreateMailslotA");
}

fn create_mailslot(emu: &mut emu::Emu, api: &str) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateMailslotW(emu: &mut emu::Emu) {
    create_mailslot(emu, "CreateMailslotW");
}
