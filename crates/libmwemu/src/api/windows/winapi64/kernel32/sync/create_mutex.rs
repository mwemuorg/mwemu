use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateMutexA(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexA", false);
}

pub fn CreateMutexW(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexW", true);
}

fn create_mutex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let attr = emu.regs().rcx;
    let owner = emu.regs().rdx;
    let name_ptr = emu.regs().r8;
    let name = strings::read_text(emu, name_ptr, wide);
    if wide {
        log_red!(emu, "kernel32!CreateMutexA '{}'", name);
    } else {
        log_red!(emu, "kernel32!{api} '{}'", name);
    }

    let uri = format!("mutex://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
