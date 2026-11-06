use crate::{emu, windows::structures};

pub fn GetVersionExW(emu: &mut emu::Emu) {
    let version_info_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!GetVersionExW 0x{:x}", version_info_ptr);
    if emu.regs().rcx != 0 && emu.maps.is_mapped(emu.regs().rcx) {
        emu.maps.write_dword(emu.regs().rcx + 4, 10);
        emu.maps.write_dword(emu.regs().rcx + 8, 0);
        emu.maps.write_dword(emu.regs().rcx + 12, 22621);
    }

    let os_version_info = structures::OsVersionInfoExW::new();
    os_version_info.save(version_info_ptr, &mut emu.maps);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
