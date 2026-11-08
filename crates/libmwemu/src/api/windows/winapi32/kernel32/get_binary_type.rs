use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetBinaryType(emu: &mut emu::Emu) {
    let app_name_ptr = arg(emu, 0) as u64;
    let binary_type_ptr = arg(emu, 1) as u64;
    let app_name = emu.maps.read_string(app_name_ptr);
    log_red!(
        emu,
        "kernel32!GetBinaryType '{}' ptr:0x{:x}",
        app_name,
        binary_type_ptr
    );
    if binary_type_ptr != 0 {
        emu.maps.write_dword(binary_type_ptr, 6);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
