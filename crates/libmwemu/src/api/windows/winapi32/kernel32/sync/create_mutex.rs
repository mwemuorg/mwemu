use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateMutexA(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexA", false);
}

fn create_mutex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let attr = WinApiAbi::arg(emu, 0) as u32;
    let owner = WinApiAbi::arg(emu, 0) as u32;
    let name_ptr = WinApiAbi::arg(emu, 0);
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
