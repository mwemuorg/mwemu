use crate::{emu, windows::structures};

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let out_sysinfo = emu.regs().rcx;

    log_red!(emu, "kernel32!GetSystemInfo sysinfo: 0x{:x}", out_sysinfo);
    if emu.regs().rcx != 0 && emu.maps.is_mapped(emu.regs().rcx) {
        emu.maps.write_word(emu.regs().rcx, 9);
        emu.maps.write_dword(emu.regs().rcx + 4, 4096);
    }

    let mut sysinfo = structures::SystemInfo64::new();
    sysinfo.save(out_sysinfo, &mut emu.maps);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
