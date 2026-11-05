use crate::emu;

pub fn LZStart(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LZStart");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
