use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateMutexA(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexA", false);
}

fn create_mutex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let attr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let owner = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);

    log_red!(emu, "kernel32!{api} '{}'", name);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    let uri = format!("mutex://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn CreateMutexW(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexW", true);
}
