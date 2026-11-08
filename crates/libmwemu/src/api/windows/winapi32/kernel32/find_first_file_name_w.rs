use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindFirstFileNameW(emu: &mut emu::Emu) {
    let filename_ptr = arg(emu, 0) as u64;
    let flags = arg(emu, 1) as u64;
    let string_length = arg(emu, 2) as u64;
    let link_name = arg(emu, 3) as u64;

    let filename = if filename_ptr != 0 {
        emu.maps.read_wide_string(filename_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "kernel32!FindFirstFileNameW file:'{}' flags:0x{:x} strlen_ptr:0x{:x} link:0x{:x}",
        filename,
        flags,
        string_length,
        link_name
    );

    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF; // INVALID_HANDLE_VALUE
    stdcall_ret(emu, 4);
}
