use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn AppXGetOSMaxVersionTested(emu: &mut emu::Emu) {
    let package_full_name = arg(emu, 0) as u64;
    let p_max_version = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!AppXGetOSMaxVersionTested name:0x{:x} pMaxVer:0x{:x}",
        package_full_name,
        p_max_version
    );

    if p_max_version != 0 {
        emu.maps.write_qword(p_max_version, 0x0A00_0000);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
