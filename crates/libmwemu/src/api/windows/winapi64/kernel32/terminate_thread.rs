use crate::emu;

pub fn TerminateThread(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    log_red!(emu, "kernel32!TerminateThread p1:0x{:x}", p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
