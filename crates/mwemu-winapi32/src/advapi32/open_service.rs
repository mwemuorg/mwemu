use crate::emu;
use crate::winapi::helper;

pub fn OpenServiceA(emu: &mut emu::Emu) {
    let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let name = emu.maps.read_string(name_ptr);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!OpenServiceA scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
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
    let hsc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let access = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let name = emu.maps.read_wide_string(name_ptr);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!OpenServiceW scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
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
