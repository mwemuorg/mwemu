use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!GetEnvironmentStrings");
    let env_str = "COMPUTERNAME=DESKTOP-MWEMU\0SYSTEMROOT=C:\\Windows\0\0";
    let handle = helper::handler_create("env_strings");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    get_environment_strings(emu, "GetEnvironmentStringsA", false);
}

fn get_environment_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    get_environment_strings(emu, "GetEnvironmentStringsW", true);
}
