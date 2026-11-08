use crate::emu;
use crate::winapi::helper;

pub fn CreateJobSet_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateJobSet");
    emu.regs_mut().rax = helper::handler_create("CreateJobSet");
}
