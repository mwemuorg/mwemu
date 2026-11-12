use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let tz_ptr = WinApiAbi::arg(emu, 0) as u32;
    let ut_ptr = WinApiAbi::arg(emu, 1) as u32;
    let lt_ptr = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!SystemTimeToTzSpecificLocalTime");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
