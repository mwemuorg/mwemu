use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateServiceA(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceA", false);
}

fn create_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hsc = WinApiAbi::arg(emu, 0) as u32;
    let name_ptr = WinApiAbi::arg(emu, 1);
    let display_ptr = WinApiAbi::arg(emu, 2);
    let access = WinApiAbi::arg(emu, 3) as u32;
    let name = strings::read_text(emu, name_ptr, wide);
    let display = strings::read_text(emu, display_ptr, wide);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!{api} '{}' display:'{}' => 0x{:x}",
        name,
        display,
        hsvc
    );
    for _ in 0..13 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = hsvc;
}

pub fn CreateServiceW(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceW", true);
}
