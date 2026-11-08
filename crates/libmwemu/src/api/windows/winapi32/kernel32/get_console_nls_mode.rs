use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleNlsMode(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let mode_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetConsoleNlsMode h:0x{:x} mode:0x{:x}",
        h_console,
        mode_ptr
    );
    if mode_ptr != 0 {
        emu.maps.write_dword(mode_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
