use crate::emu;

pub fn SHGetFolderPathAndSubDirA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let csidl = emu.regs().rdx as i32;
    let htoken = emu.regs().r8;
    let dw_flags = emu.regs().r9;
    let psz_sub_dir = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let psz_path = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let sub_dir = if psz_sub_dir != 0 {
        emu.maps.read_string(psz_sub_dir)
    } else {
        String::new()
    };
    let path = format!("C:\\Users\\user\\AppData\\{}", sub_dir);
    if psz_path != 0 {
        emu.maps.write_string(psz_path, &path);
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathAndSubDirA csidl:0x{:x} sub:'{}' -> '{}'",
        csidl,
        sub_dir,
        path
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
