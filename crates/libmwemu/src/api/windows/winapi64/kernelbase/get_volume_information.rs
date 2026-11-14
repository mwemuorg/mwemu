use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
    let root_ptr = emu.regs().rcx;
    let vol_name_ptr = emu.regs().rdx;
    let vol_name_size = emu.regs().r8;
    let serial_ptr = emu.regs().r9;
    let max_comp_ptr = WinApiAbi::arg(emu, 4);
    let flags_ptr = WinApiAbi::arg(emu, 5);
    let fs_name_ptr = WinApiAbi::arg(emu, 6);
    let fs_name_size = WinApiAbi::arg(emu, 7);
    let root = if root_ptr != 0 {
        emu.maps.read_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    if vol_name_ptr != 0 && vol_name_size > 0 {
        emu.maps.write_string(vol_name_ptr, "MWEMU");
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
        emu.maps.write_string(fs_name_ptr, "NTFS");
    }
    log_red!(emu, "kernelbase!GetVolumeInformationA '{}'", root);
    emu.regs_mut().rax = 1;
}

pub fn GetVolumeInformationW(emu: &mut emu::Emu) {
    let root = emu.regs().rcx;
    let vol_name_buf = emu.regs().rdx;
    let vol_name_sz = emu.regs().r8 as u32;
    let serial_ptr = emu.regs().r9;
    let max_comp_ptr = WinApiAbi::arg(emu, 4);
    let flags_ptr = WinApiAbi::arg(emu, 5);
    let fs_name_buf = WinApiAbi::arg(emu, 6);
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
