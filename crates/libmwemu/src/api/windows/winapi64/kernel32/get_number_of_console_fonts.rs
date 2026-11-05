use crate::emu;

pub fn GetNumberOfConsoleFonts(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetNumberOfConsoleFonts => 1");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
