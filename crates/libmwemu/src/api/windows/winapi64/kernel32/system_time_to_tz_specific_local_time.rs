use crate::emu;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let tz_ptr = emu.regs().rcx;
    let ut_ptr = emu.regs().rcx;
    let lt_ptr = emu.regs().r8;

    log_red!(emu, "kernel32!SystemTimeToTzSpecificLocalTime");

    emu.regs_mut().rax = 1;
}
