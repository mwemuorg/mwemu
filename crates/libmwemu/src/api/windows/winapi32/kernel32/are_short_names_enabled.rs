use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn AreShortNamesEnabled(emu: &mut emu::Emu) {
    let h_volume = arg(emu, 0) as u64;
    let p_enabled = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!AreShortNamesEnabled hVolume:0x{:x} pEnabled:0x{:x}",
        h_volume,
        p_enabled
    );
    if p_enabled != 0 {
        emu.maps.write_dword(p_enabled, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
