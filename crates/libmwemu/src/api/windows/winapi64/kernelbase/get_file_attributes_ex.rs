use crate::emu;
use crate::winapi::common::strings;

pub fn GetFileAttributesExA(emu: &mut emu::Emu) {
    get_file_attributes_ex(emu, "GetFileAttributesExA", false);
}

fn get_file_attributes_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let info_level = emu.regs().rdx;
    let file_info_ptr = emu.regs().r8;
    let path = strings::read_text(emu, path_ptr, wide);
    if file_info_ptr != 0 {
        emu.maps.write_dword(file_info_ptr, 0x80); // dwFileAttributes = NORMAL
        emu.maps.write_dword(file_info_ptr + 28, 0); // nFileSizeHigh
        emu.maps.write_dword(file_info_ptr + 32, 4096); // nFileSizeLow
    }
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.regs_mut().rax = 1;
}

pub fn GetFileAttributesExW(emu: &mut emu::Emu) {
    get_file_attributes_ex(emu, "GetFileAttributesExW", true);
}
