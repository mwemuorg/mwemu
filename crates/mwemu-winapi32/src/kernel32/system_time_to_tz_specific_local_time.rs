use crate::emu;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let tz_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let ut_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!SystemTimeToTzSpecificLocalTime");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
