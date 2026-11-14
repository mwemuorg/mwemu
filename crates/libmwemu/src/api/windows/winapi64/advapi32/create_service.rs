use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateServiceA(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceA", false);
}

fn create_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hsc = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let display_ptr = emu.regs().r8;
    let access = emu.regs().r9;
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
    emu.set_last_error(0);
    emu.regs_mut().rax = hsvc;
}

pub fn CreateServiceW(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceW", true);
}
