use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileAttributesExA(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let _info_level = arg(emu, 1);
    let file_info = arg(emu, 2) as u64;

    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "kernel32!GetFileAttributesExA name:'{}'", name);

    if file_info != 0 {
        // WIN32_FILE_ATTRIBUTE_DATA (36 bytes)
        let attrs: u32 = if name.ends_with('\\') || name.ends_with('/') {
            0x10 // FILE_ATTRIBUTE_DIRECTORY
        } else {
            0x80 // FILE_ATTRIBUTE_NORMAL
        };
        emu.maps.write_dword(file_info, attrs);
        emu.maps.write_qword(file_info + 4, 0x01DA_5E3E_A46C_0000);
        emu.maps.write_qword(file_info + 12, 0x01DA_5E3E_A46C_0000);
        emu.maps.write_qword(file_info + 20, 0x01DA_5E3E_A46C_0000);
        emu.maps.write_dword(file_info + 28, 0);
        emu.maps.write_dword(file_info + 32, 4096);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
