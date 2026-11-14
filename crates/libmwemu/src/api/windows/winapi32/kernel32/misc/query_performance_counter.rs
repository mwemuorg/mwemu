use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn QueryPerformanceCounter(emu: &mut emu::Emu) {
    let counter_ptr = WinApiAbi::arg(emu, 0);

    emu.maps.write_dword(counter_ptr, 0x1);

    log_red!(emu, "kernel32!QueryPerformanceCounter");

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
