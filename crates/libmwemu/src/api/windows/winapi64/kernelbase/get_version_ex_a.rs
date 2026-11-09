use crate::emu;

pub fn GetVersionExA(emu: &mut emu::Emu) {
    let info_ptr = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetVersionExA ptr:0x{:x}", info_ptr);
    if info_ptr != 0 {
        let size = emu.maps.read_dword(info_ptr).unwrap_or(148);
        emu.maps.write_dword(info_ptr, size);
        emu.maps.write_dword(info_ptr + 4, 10); // major
        emu.maps.write_dword(info_ptr + 8, 0); // minor
        emu.maps.write_dword(info_ptr + 12, 19045); // build
        emu.maps.write_dword(info_ptr + 16, 2); // VER_PLATFORM_WIN32_NT
    }
    emu.regs_mut().rax = 1;
}
