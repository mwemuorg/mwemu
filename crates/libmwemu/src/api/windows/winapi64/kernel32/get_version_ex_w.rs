use crate::{emu, windows::structures};

pub fn GetVersionExW(emu: &mut emu::Emu) {
    let version_info_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!GetVersionExW 0x{:x}", version_info_ptr);

    let os_version_info = structures::OsVersionInfoExW::new();
    os_version_info.save(version_info_ptr, &mut emu.maps);

    emu.regs_mut().rax = 1;
}
