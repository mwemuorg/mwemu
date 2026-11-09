use crate::emu;
use crate::winapi::helper;

pub fn CreateServiceA(emu: &mut emu::Emu) {
    let hsc = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let display_ptr = emu.regs().r8;
    let access = emu.regs().r9;
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
    emu.set_last_error(0);
    emu.regs_mut().rax = hsvc;
}

pub fn CreateServiceW(emu: &mut emu::Emu) {
    let hsc = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let display_ptr = emu.regs().r8;
    let access = emu.regs().r9;
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
    emu.set_last_error(0);
    emu.regs_mut().rax = hsvc;
}
