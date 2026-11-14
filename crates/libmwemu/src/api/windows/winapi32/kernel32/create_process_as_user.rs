use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserA");
}

fn create_process_as_user(emu: &mut emu::Emu, api: &str) {
    for _ in 0..11u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserW");
}
