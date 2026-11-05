use crate::emu;

pub fn IsTerminalServerCompatible(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsTerminalServerCompatible => FALSE");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
