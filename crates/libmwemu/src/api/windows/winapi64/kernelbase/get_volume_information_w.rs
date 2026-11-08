use crate::emu;

pub fn GetVolumeInformationW(emu: &mut emu::Emu) {
    let root = emu.regs().rcx;
    let vol_name_buf = emu.regs().rdx;
    let vol_name_sz = emu.regs().r8 as u32;
    let serial_ptr = emu.regs().r9;
    let max_comp_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let fs_name_buf = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    log_red!(emu, "kernelbase!GetVolumeInformationW");
    let vol = "Windows";
    if vol_name_buf != 0 && vol_name_sz > 0 {
        for (i, ch) in vol.encode_utf16().enumerate() {
            emu.maps.write_word(vol_name_buf + (i as u64) * 2, ch);
        }
        emu.maps
            .write_word(vol_name_buf + (vol.len() as u64) * 2, 0);
    }
    if serial_ptr != 0 {
        emu.maps.write_dword(serial_ptr, 0x12345678);
    }
    if max_comp_ptr != 0 {
        emu.maps.write_dword(max_comp_ptr, 255);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x000700FF);
    }
    let fs = "NTFS";
    if fs_name_buf != 0 {
        for (i, ch) in fs.encode_utf16().enumerate() {
            emu.maps.write_word(fs_name_buf + (i as u64) * 2, ch);
        }
        emu.maps.write_word(fs_name_buf + (fs.len() as u64) * 2, 0);
    }
    emu.regs_mut().rax = 1; // TRUE
}
