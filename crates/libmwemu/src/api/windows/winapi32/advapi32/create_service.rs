use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateServiceA(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceA", false);
}

fn create_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let display_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let access = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
