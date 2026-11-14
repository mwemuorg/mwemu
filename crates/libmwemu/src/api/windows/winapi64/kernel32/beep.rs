use crate::emu;

pub fn Beep(emu: &mut emu::Emu) {
    let freq = emu.regs().rcx;
    let duration = emu.regs().rdx;
    log_red!(emu, "kernel32!Beep freq:{} dur:{}", freq, duration);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
