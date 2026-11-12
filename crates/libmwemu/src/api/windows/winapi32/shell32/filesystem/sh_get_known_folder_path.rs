use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetKnownFolderPath(emu: &mut emu::Emu) {
    let folder_id_ptr = WinApiAbi::arg(emu, 0);
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let token = WinApiAbi::arg(emu, 2) as u32;
    let path_out_ptr = WinApiAbi::arg(emu, 3);
    let path = "C:\\Users\\mwemu_user\\AppData\\Local";
    let alloc_addr = emu
        .maps
        .alloc(512)
        .expect("shell32!SHGetKnownFolderPath alloc");
    emu.maps.write_wide_string(alloc_addr, path);
    if path_out_ptr != 0 {
        emu.maps.write_dword(path_out_ptr, alloc_addr as u32);
    }
    log_red!(
        emu,
        "shell32!SHGetKnownFolderPath flags:0x{:x} => '{}'",
        flags,
        path
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
