use crate::emu;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let file_time = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let sys_time_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!FileTimeToSystemTime");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
