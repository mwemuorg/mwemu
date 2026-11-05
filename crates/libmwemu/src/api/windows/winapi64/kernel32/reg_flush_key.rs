use crate::emu;

pub fn RegFlushKey(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernel32!RegFlushKey");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
