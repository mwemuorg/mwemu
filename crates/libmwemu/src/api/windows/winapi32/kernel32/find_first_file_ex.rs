use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileExA(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExA");
}

fn find_first_file_ex(emu: &mut emu::Emu, api: &str) {
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn FindFirstFileExW(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExW");
}
