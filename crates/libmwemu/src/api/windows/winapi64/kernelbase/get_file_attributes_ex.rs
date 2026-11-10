use crate::emu;

pub fn GetFileAttributesExA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let info_level = emu.regs().rdx;
    let file_info_ptr = emu.regs().r8;
    let path = emu.maps.read_string(path_ptr);
    if file_info_ptr != 0 {
        emu.maps.write_dword(file_info_ptr, 0x80); // dwFileAttributes = NORMAL
        emu.maps.write_dword(file_info_ptr + 28, 0); // nFileSizeHigh
        emu.maps.write_dword(file_info_ptr + 32, 4096); // nFileSizeLow
    }
    log_red!(emu, "kernelbase!GetFileAttributesExA '{}'", path);
    emu.regs_mut().rax = 1;
}

pub fn GetFileAttributesExW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let info_level = emu.regs().rdx;
    let file_info_ptr = emu.regs().r8;
    let path = emu.maps.read_wide_string(path_ptr);
    if file_info_ptr != 0 {
        emu.maps.write_dword(file_info_ptr, 0x80);
        emu.maps.write_dword(file_info_ptr + 28, 0);
        emu.maps.write_dword(file_info_ptr + 32, 4096);
    }
    log_red!(emu, "kernelbase!GetFileAttributesExW '{}'", path);
    emu.regs_mut().rax = 1;
}
