use crate::{emu, windows::structures};

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let ptr_sysinfo = emu.regs().rcx;

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(ptr_sysinfo, &mut emu.maps);

    log_red!(emu, "kernel32!GetNativeSysteminfo {:?}", sysinfo);

    log_red!(emu, "kernel32!GetNativeSysteminfo 0x{:x}", ptr_sysinfo);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
