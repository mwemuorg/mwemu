use crate::emu;

pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
    let sp = emu.regs().get_esp();
    let root_ptr = emu.maps.read_dword(sp).unwrap_or(0) as u64;
    let name_buf = emu.maps.read_dword(sp + 4).unwrap_or(0) as u64;
    let name_size = emu.maps.read_dword(sp + 8).unwrap_or(0);
    let serial = emu.maps.read_dword(sp + 12).unwrap_or(0) as u64;
    let max_comp = emu.maps.read_dword(sp + 16).unwrap_or(0) as u64;
    let fs_flags = emu.maps.read_dword(sp + 20).unwrap_or(0) as u64;
    let fs_name_buf = emu.maps.read_dword(sp + 24).unwrap_or(0) as u64;
    let fs_name_size = emu.maps.read_dword(sp + 28).unwrap_or(0);

    let root = if root_ptr != 0 {
        emu.maps.read_string(root_ptr)
    } else {
        String::new()
    };

    if name_buf != 0 && name_size > 8 {
        emu.maps.write_string(name_buf, "Windows");
    }
    if serial != 0 {
        emu.maps.write_dword(serial, 0x1234_5678);
    }
    if max_comp != 0 {
        emu.maps.write_dword(max_comp, 255);
    }
    if fs_flags != 0 {
        emu.maps.write_dword(fs_flags, 0x000700FF);
    }
    if fs_name_buf != 0 && fs_name_size > 5 {
        emu.maps.write_string(fs_name_buf, "NTFS");
    }

    log_red!(emu, "kernel32!GetVolumeInformationA '{}'", root);

    for _ in 0..8 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
