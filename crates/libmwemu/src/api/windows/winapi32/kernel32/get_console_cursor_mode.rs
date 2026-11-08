use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleCursorMode(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let mode_ptr = arg(emu, 1) as u64;
    let visible_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetConsoleCursorMode h:0x{:x} mode:0x{:x} vis:0x{:x}",
        h_console,
        mode_ptr,
        visible_ptr
    );
    if mode_ptr != 0 {
        emu.maps.write_dword(mode_ptr, 0);
    }
    if visible_ptr != 0 {
        emu.maps.write_dword(visible_ptr, 1);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
