use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCurrentPackageInfo(emu: &mut emu::Emu) {
    let flags = arg(emu, 0) as u64;
    let buffer_length_ptr = arg(emu, 1) as u64;
    let _buffer_ptr = arg(emu, 2) as u64;
    let count_ptr = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!GetCurrentPackageInfo flags:0x{:x} len_ptr:0x{:x}",
        flags,
        buffer_length_ptr
    );

    if buffer_length_ptr != 0 {
        emu.maps.write_dword(buffer_length_ptr, 0);
    }
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
    stdcall_ret(emu, 4);
}
