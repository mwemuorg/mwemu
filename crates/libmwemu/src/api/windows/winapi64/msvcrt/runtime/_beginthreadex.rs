use crate::emu;
use crate::winapi::helper;

pub fn _beginthreadex(emu: &mut emu::Emu) {
    let _security = emu.regs().rcx;
    let _stack_size = emu.regs().rdx;
    let start = emu.regs().r8;
    log_red!(emu, "msvcrt!_beginthreadex start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthreadex");
    emu.regs_mut().rax = hndl;
}
