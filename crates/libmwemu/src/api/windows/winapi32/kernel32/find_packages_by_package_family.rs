use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindPackagesByPackageFamily(emu: &mut emu::Emu) {
    let family_name_ptr = arg(emu, 0) as u64;
    let flags = arg(emu, 1) as u64 as u32;
    let count_ptr = arg(emu, 2) as u64;
    let names_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!FindPackagesByPackageFamily name:0x{:x} flags:0x{:x}",
        family_name_ptr,
        flags
    );
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
