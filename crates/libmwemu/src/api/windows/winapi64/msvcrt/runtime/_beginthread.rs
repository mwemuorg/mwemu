use crate::emu;
use crate::winapi::helper;

pub fn _beginthread(emu: &mut emu::Emu) {
    let start = emu.regs().rcx;
    log_red!(emu, "msvcrt!_beginthread start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthread");
    emu.regs_mut().rax = hndl;
}
