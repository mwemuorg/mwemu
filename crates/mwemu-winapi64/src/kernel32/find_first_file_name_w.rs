use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileNameW(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let string_length = emu.regs().r8;
    let link_name = emu.regs().r9;

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
}
