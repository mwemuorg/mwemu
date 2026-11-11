use crate::emu;
use crate::winapi::helper;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!GetEnvironmentStrings");
    let env_str = "COMPUTERNAME=DESKTOP-MWEMU\0SYSTEMROOT=C:\\Windows\0\0";
    let handle = helper::handler_create("env_strings");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_string(str_ptr);
    log_red!(
        emu,
        "kernelbase!GetEnvironmentStringsA '{}' 0x{:x}",
        name,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernelbase!GetEnvironmentStringsW '{}' 0x{:x}",
        name,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
