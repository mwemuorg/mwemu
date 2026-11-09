use crate::emu;
use crate::windows::constants::errors::*;

pub fn BCryptFinalizeKeyPair(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;

    log_red!(emu, "bcrypt!BCryptFinalizeKeyPair key=0x{:x}", h_key);

    emu.regs_mut().rax = STATUS_SUCCESS;
}
