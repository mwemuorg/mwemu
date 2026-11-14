use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let file_time = WinApiAbi::arg(emu, 0) as u32;
    let sys_time_ptr = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!FileTimeToSystemTime");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
