use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x} flags: 0x{:x}",
        h_algorithm,
        flags
    );

    helper::handler_close(h_algorithm);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
