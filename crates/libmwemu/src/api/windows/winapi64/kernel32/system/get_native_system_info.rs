use crate::{emu, windows::structures};

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let ptr_sysinfo = emu.regs().rcx;

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(ptr_sysinfo, &mut emu.maps);

    log_red!(emu, "kernel32!GetNativeSysteminfo {:?}", sysinfo);
    if emu.regs().rcx != 0 && emu.maps.is_mapped(emu.regs().rcx) {
        emu.maps.write_word(emu.regs().rcx, 9);
        emu.maps.write_dword(emu.regs().rcx + 4, 4096);
    }

    log_red!(emu, "kernel32!GetNativeSysteminfo 0x{:x}", ptr_sysinfo);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
