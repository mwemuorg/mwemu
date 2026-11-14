use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileTransactedA(emu: &mut emu::Emu) {
    find_first_file_transacted(emu, "FindFirstFileTransactedA");
}

fn find_first_file_transacted(emu: &mut emu::Emu, api: &str) {
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn FindFirstFileTransactedW(emu: &mut emu::Emu) {
    find_first_file_transacted(emu, "FindFirstFileTransactedW");
}
