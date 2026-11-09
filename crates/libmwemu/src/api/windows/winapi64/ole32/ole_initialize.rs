use crate::emu;
use crate::windows::constants;

pub fn OleInitialize(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "ole32!OleInitialize arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::S_OK;
}
