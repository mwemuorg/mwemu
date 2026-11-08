use crate::emu;
use crate::winapi::helper;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!GetEnvironmentStrings");
    let env_str = "COMPUTERNAME=DESKTOP-MWEMU\0SYSTEMROOT=C:\\Windows\0\0";
    let handle = helper::handler_create("env_strings");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
