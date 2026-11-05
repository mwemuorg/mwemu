use crate::emu;

pub fn InitOnceInitialize(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernel32!InitOnceInitialize");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
