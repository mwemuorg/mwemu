use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCurrentPackageFullName(emu: &mut emu::Emu) {
    let length_ptr = arg(emu, 0) as u64;
    let name_ptr = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!GetCurrentPackageFullName length_ptr:0x{:x} name_ptr:0x{:x}",
        length_ptr,
        name_ptr
    );

    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
    stdcall_ret(emu, 2);
}
