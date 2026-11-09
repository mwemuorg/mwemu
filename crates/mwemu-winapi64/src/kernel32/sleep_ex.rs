use crate::emu;

pub fn SleepEx(emu: &mut emu::Emu) {
    let ms = emu.regs().rcx;
    let alertable = emu.regs().rdx;
    log_red!(emu, "kernel32!SleepEx ms:{} alertable:{}", ms, alertable);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
