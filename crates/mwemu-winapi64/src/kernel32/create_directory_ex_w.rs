use crate::emu;
use crate::winapi::helper;

pub fn CreateDirectoryExW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateDirectoryExW");
    emu.regs_mut().rax = helper::handler_create("CreateDirectoryExW");
}
