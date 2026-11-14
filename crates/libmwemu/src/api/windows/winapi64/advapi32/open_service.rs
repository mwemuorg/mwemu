use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn OpenServiceA(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceA", false);
}

fn open_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hsc = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let access = emu.regs().r8;
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
    emu.set_last_error(0);
    emu.regs_mut().rax = hsvc;
}

pub fn OpenServiceW(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceW", true);
}
