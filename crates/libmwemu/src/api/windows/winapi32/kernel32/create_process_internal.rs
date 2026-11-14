use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessInternalA(emu: &mut emu::Emu) {
    create_process_internal(emu, "CreateProcessInternalA");
}

fn create_process_internal(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn CreateProcessInternalW(emu: &mut emu::Emu) {
    create_process_internal(emu, "CreateProcessInternalW");
}
