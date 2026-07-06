use crate::emu;

pub fn GetACP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetACP");
    emu.regs_mut().rax = 0x00000409;
}
