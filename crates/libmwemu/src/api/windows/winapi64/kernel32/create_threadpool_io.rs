use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolIo(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolIo");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolIo");
}
