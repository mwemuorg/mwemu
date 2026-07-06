use crate::emu;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let sys_time_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetSystemTimeAsFileTime cannot read sys_time_ptr");

    log_red!(emu, "kernel32!GetSystemTimeAsFileTime");

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
