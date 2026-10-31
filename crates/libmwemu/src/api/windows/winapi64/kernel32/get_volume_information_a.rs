use crate::emu;

pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
    let root = emu.maps.read_string(emu.regs().rcx);
    let name_buf = emu.regs().rdx;
    let name_size = emu.regs().r8;
    let serial = emu.regs().r9;

    if name_buf != 0 && name_size > 8 {
        emu.maps.write_string(name_buf, "Windows");
    }
    if serial != 0 {
        emu.maps.write_dword(serial, 0x1234_5678);
    }

    let sp = emu.regs().rsp;
    let max_comp = emu.maps.read_qword(sp + 0x28).unwrap_or(0);
    if max_comp != 0 {
        emu.maps.write_dword(max_comp, 255);
    }
    let fs_flags = emu.maps.read_qword(sp + 0x30).unwrap_or(0);
    if fs_flags != 0 {
        emu.maps.write_dword(fs_flags, 0x000700FF);
    }
    let fs_name_buf = emu.maps.read_qword(sp + 0x38).unwrap_or(0);
    let fs_name_size = emu.maps.read_qword(sp + 0x40).unwrap_or(0);
    if fs_name_buf != 0 && fs_name_size > 5 {
        emu.maps.write_string(fs_name_buf, "NTFS");
    }

    log_red!(emu, "kernel32!GetVolumeInformationA '{}'", root);
    emu.regs_mut().rax = 1;
}
