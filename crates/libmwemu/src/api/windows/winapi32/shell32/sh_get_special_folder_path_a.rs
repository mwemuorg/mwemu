use crate::emu;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let path_buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let csidl = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let create = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let path = match csidl {
        0x001a => "C:\\Users\\mwemu_user\\AppData\\Roaming",
        0x001c => "C:\\Users\\mwemu_user\\AppData\\Local",
        0x0024 => "C:\\Windows",
        0x0025 => "C:\\Windows\\System32",
        _ => "C:\\Users\\mwemu_user",
    };
    if path_buf != 0 {
        emu.maps.write_string(path_buf, path);
    }
    log_red!(
        emu,
        "shell32!SHGetSpecialFolderPathA csidl:0x{:x} => '{}'",
        csidl,
        path
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
