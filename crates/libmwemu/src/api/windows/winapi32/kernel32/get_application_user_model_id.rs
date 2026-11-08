use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetApplicationUserModelId(emu: &mut emu::Emu) {
    let process = arg(emu, 0) as u64;
    let length_ptr = arg(emu, 1) as u64;
    let app_id_ptr = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!GetApplicationUserModelId proc:0x{:x} length_ptr:0x{:x}",
        process,
        length_ptr
    );

    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
    stdcall_ret(emu, 3);
}
