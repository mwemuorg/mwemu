use crate::emu;
use crate::winapi::helper;

pub fn CreateFileTransactedA(emu: &mut emu::Emu) {
    create_file_transacted(emu, "CreateFileTransactedA");
}

fn create_file_transacted(emu: &mut emu::Emu, api: &str) {
    for _ in 0..10u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateFileTransactedW(emu: &mut emu::Emu) {
    create_file_transacted(emu, "CreateFileTransactedW");
}
