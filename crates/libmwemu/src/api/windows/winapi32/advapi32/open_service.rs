use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn OpenServiceA(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceA", false);
}

fn open_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hsc = WinApiAbi::arg(emu, 0) as u32;
    let name_ptr = WinApiAbi::arg(emu, 1);
    let access = WinApiAbi::arg(emu, 2) as u32;
    let name = strings::read_text(emu, name_ptr, wide);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!{api} scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
        hsc,
        name,
        access,
        hsvc
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = hsvc;
}

pub fn OpenServiceW(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceW", true);
}
