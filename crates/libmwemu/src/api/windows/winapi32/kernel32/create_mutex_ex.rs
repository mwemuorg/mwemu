use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexExA(emu: &mut emu::Emu) {
    create_mutex_ex(emu, "CreateMutexExA");
}

fn create_mutex_ex(emu: &mut emu::Emu, api: &str) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateMutexExW(emu: &mut emu::Emu) {
    create_mutex_ex(emu, "CreateMutexExW");
}
