use crate::emu;

pub fn timeEndPeriod(emu: &mut emu::Emu) {
    let period = emu.regs().rcx;

    log_red!(emu, "kernel32!timeEndPeriod period:{}", period);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
