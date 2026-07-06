use crate::emu;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let file_time = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FileTimeToSystemTime cannot read file_time");
    let sys_time_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FileTimeToSystemTime cannot read sys_time_ptr");

    log_red!(emu, "kernel32!FileTimeToSystemTime");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
