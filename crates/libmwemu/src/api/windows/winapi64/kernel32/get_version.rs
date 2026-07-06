use crate::{emu, windows::constants};

pub fn GetVersion(emu: &mut emu::Emu) {
    emu.regs_mut().rax = constants::VERSION;
    log_red!(emu, "kernel32!GetVersion   =0x{:x}", emu.regs().rax);
}
