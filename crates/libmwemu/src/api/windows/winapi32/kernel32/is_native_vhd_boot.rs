use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsNativeVhdBoot(emu: &mut emu::Emu) {
    let p_native_vhd_boot = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!IsNativeVhdBoot pNativeVhdBoot:0x{:x}",
        p_native_vhd_boot
    );
    if p_native_vhd_boot != 0 {
        emu.maps.write_dword(p_native_vhd_boot, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
