use crate::emu;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let sys_time_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!GetSystemTimeAsFileTime");

    // TODO: implement

    emu.regs_mut().rax = 1;
}
