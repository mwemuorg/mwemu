use crate::emu;

pub fn SHGetFolderPathA(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let path = match csidl {
        0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
        0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
        0x0024 => "C:\\Windows",
        0x0025 => "C:\\Windows\\System32",
        0x0027 => "C:\\Program Files",
        0x002b => "C:\\Program Files (x86)",
        0x0005 => "C:\\Users\\mwemu_user\\Documents",
        0x0028 => "C:\\Users\\mwemu_user\\Desktop",
        _ => "C:\\Users\\mwemu_user",
    };
    if path_buf != 0 {
        emu.maps.write_string(path_buf, path);
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathA csidl:0x{:x} => '{}'",
        csidl,
        path
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
