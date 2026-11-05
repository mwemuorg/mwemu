use crate::emu;

pub fn SetSystemTime(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernel32!SetSystemTime");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
