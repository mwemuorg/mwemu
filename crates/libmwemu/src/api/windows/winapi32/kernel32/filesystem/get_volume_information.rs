use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
    get_volume_information(emu, "GetVolumeInformationA", false);
}

fn get_volume_information(emu: &mut emu::Emu, api: &str, wide: bool) {
    let root_ptr = WinApiAbi::arg(emu, 0);
    let name_buf = WinApiAbi::arg(emu, 1);
    let name_size = WinApiAbi::arg(emu, 2) as u32;
    let serial = WinApiAbi::arg(emu, 3);
    let max_comp = WinApiAbi::arg(emu, 4);
    let fs_flags = WinApiAbi::arg(emu, 5);
    let fs_name_buf = WinApiAbi::arg(emu, 6);
    let fs_name_size = WinApiAbi::arg(emu, 7) as u32;

    let root = if root_ptr != 0 {
        strings::read_text(emu, root_ptr, wide)
    } else {
        String::new()
    };

    if name_buf != 0 && name_size > 8 {
        strings::write_string(emu, name_buf, "Windows", wide);
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
        strings::write_string(emu, fs_name_buf, "NTFS", wide);
    }

    log_red!(emu, "kernel32!{api} '{}'", root);

    for _ in 0..8 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

pub fn GetVolumeInformationW(emu: &mut emu::Emu) {
    get_volume_information(emu, "GetVolumeInformationW", true);
}
