use crate::emu;

pub fn GetVolumeInformationByHandleW(emu: &mut emu::Emu) {
    let root_ptr = emu.regs().rcx;
    let vol_name_ptr = emu.regs().rdx;
    let vol_name_size = emu.regs().r8;
    let serial_ptr = emu.regs().r9;
    let max_comp_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let fs_name_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let fs_name_size = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let root = if root_ptr != 0 {
        emu.maps.read_wide_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    if vol_name_ptr != 0 && vol_name_size > 0 {
        emu.maps.write_wide_string(vol_name_ptr, "MWEMU");
    }
    if serial_ptr != 0 {
        emu.maps.write_dword(serial_ptr, 0x12345678);
    }
    if max_comp_ptr != 0 {
        emu.maps.write_dword(max_comp_ptr, 255);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x700ff);
    }
    if fs_name_ptr != 0 && fs_name_size > 0 {
        emu.maps.write_wide_string(fs_name_ptr, "NTFS");
    }
    log_red!(emu, "kernelbase!GetVolumeInformationByHandleW '{}'", root);
    emu.regs_mut().rax = 1;
}
