use crate::emu;

pub fn GetFileAttributesExW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let _info_level = emu.regs().rdx as u32;
    let file_info = emu.regs().r8;

    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "kernel32!GetFileAttributesExW name:'{}'", name);

    if file_info != 0 {
        let attrs: u32 = if name.ends_with('\\') || name.ends_with('/') {
            0x10
        } else {
            0x80
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
}
