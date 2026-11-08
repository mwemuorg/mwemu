use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNLSVersion(emu: &mut emu::Emu) {
    let function = arg(emu, 0);
    let locale = arg(emu, 1);
    let version_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetNLSVersion func:{} locale:0x{:x} ver:0x{:x}",
        function,
        locale,
        version_ptr
    );
    if version_ptr != 0 {
        emu.maps.write_dword(version_ptr, 32);
        emu.maps.write_dword(version_ptr + 4, 1);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
