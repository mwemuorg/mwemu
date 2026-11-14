use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateSemaphoreA(emu: &mut emu::Emu) {
    create_semaphore(emu, "CreateSemaphoreA", false);
}

pub fn CreateSemaphoreW(emu: &mut emu::Emu) {
    create_semaphore(emu, "CreateSemaphoreW", true);
}

fn create_semaphore(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _security = WinApiAbi::arg(emu, 0);
    let initial = WinApiAbi::arg(emu, 1);
    let maximum = WinApiAbi::arg(emu, 2);
    let name = WinApiAbi::arg(emu, 3);
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!{} init:{} max:{} name:'{}'",
        api,
        initial,
        maximum,
        name_s
    );
    emu.regs_mut().rax = helper::handler_create(&name_s);
}
