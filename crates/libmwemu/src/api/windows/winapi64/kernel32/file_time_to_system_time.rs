use crate::emu;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let file_time = emu.regs().rcx;
    let sys_time_ptr = emu.regs().rdx;

    log_red!(emu, "kernel32!FileTimeToSystemTime");
    emu.regs_mut().rax = 1;
}
