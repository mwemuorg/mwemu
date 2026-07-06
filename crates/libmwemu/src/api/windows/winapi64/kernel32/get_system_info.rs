use crate::{emu, windows::structures};

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = emu.regs().rcx;

    log_red!(emu, "kernel32!GetSystemInfo sysinfo: 0x{:x}", out_sysinfo);

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(out_sysinfo, &mut emu.maps);
}
