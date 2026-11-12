use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetPathFromIDList(emu: &mut emu::Emu) {
    let pidl = WinApiAbi::arg(emu, 0);
    let psz_path = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "shell32!SHGetPathFromIDList pidl:0x{:x} path:0x{:x}",
        pidl,
        psz_path
    );
    if psz_path != 0 {
        emu.maps.write_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn SHGetPathFromIDListA(emu: &mut emu::Emu) {
    sh_get_path_from_id_list(emu, "SHGetPathFromIDListA", false);
}

fn sh_get_path_from_id_list(emu: &mut emu::Emu, api: &str, wide: bool) {
    let pidl = WinApiAbi::arg(emu, 0);
    let psz_path = WinApiAbi::arg(emu, 1);
    log_red!(emu, "shell32!{api} pidl:0x{:x} path:0x{:x}", pidl, psz_path);
    if psz_path != 0 {
        strings::write_string(emu, psz_path, "C:\\Users\\User\\Desktop", wide);
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn SHGetPathFromIDListW(emu: &mut emu::Emu) {
    sh_get_path_from_id_list(emu, "SHGetPathFromIDListW", true);
}
