use super::csidl_to_path;
use crate::emu;

pub fn SHGetFolderPathA(emu: &mut emu::Emu) {
    let csidl = emu.regs().rdx as i32;
    let psz_path = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(emu, "shell32!SHGetFolderPathA csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                emu.maps.write_string(psz_path, path);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 42; // S_OK
        }
        None => {
            log::warn!("SHGetFolderPathA: unsupported CSIDL 0x{:x}", csidl);
            emu.regs_mut().rax = 0x80070002;
        }
    }
}
