use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleCursorInfo(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let cursor_info_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetConsoleCursorInfo h:0x{:x} ptr:0x{:x}",
        h_console,
        cursor_info_ptr
    );
    if cursor_info_ptr != 0 {
        emu.maps.write_dword(cursor_info_ptr, 25);
        emu.maps.write_dword(cursor_info_ptr + 4, 1);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
