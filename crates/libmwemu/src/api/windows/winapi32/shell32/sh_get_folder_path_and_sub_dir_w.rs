use crate::emu;

pub fn SHGetFolderPathAndSubDirW(emu: &mut emu::Emu) {
    let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let csidl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    if path_ptr != 0 {
        emu.maps.write_wide_string(path_ptr, "C:\\Users\\mwemu");
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathAndSubDirW csidl:0x{:x} => C:\\Users\\mwemu",
        csidl
    );
    emu.regs_mut().rax = 0;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
