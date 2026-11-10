use crate::emu;
use crate::winapi::helper;

pub fn CreateServiceA(emu: &mut emu::Emu) {
    let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let display_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let access = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let name = emu.maps.read_string(name_ptr);
    let display = emu.maps.read_string(display_ptr);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!CreateServiceA '{}' display:'{}' => 0x{:x}",
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
    let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let display_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let access = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let name = emu.maps.read_wide_string(name_ptr);
    let display = emu.maps.read_wide_string(display_ptr);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!CreateServiceW '{}' display:'{}' => 0x{:x}",
        name,
        display,
        hsvc
    );
    for _ in 0..13 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = hsvc;
}
