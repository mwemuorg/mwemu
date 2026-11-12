use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn OpenServiceA(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceA", false);
}

fn open_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
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
