use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let sys_time_ptr = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!GetSystemTimeAsFileTime");

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
