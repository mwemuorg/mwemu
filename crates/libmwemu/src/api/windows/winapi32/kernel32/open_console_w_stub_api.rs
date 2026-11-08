use crate::emu;
use crate::winapi::helper;

pub fn OpenConsoleWStub_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenConsoleWStub");
    emu.regs_mut().rax = helper::handler_create("OpenConsoleWStub");
}
