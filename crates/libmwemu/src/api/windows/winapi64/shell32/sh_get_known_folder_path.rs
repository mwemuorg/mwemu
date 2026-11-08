use crate::emu;

pub fn SHGetKnownFolderPath(emu: &mut emu::Emu) {
    let folder_id_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let token = emu.regs().r8;
    let path_out_ptr = emu.regs().r9;
    let path = "C:\\Users\\mwemu_user\\AppData\\Local";
    let alloc_addr = emu
        .maps
        .alloc(512)
        .expect("shell32!SHGetKnownFolderPath alloc");
    emu.maps.write_wide_string(alloc_addr, path);
    if path_out_ptr != 0 {
        emu.maps.write_qword(path_out_ptr, alloc_addr);
    }
    log_red!(
        emu,
        "shell32!SHGetKnownFolderPath flags:0x{:x} => '{}'",
        flags,
        path
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
