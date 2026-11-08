use super::csidl_to_path;
use crate::emu;

pub fn SHGetSpecialFolderPathA(emu: &mut emu::Emu) {
    let psz_path = emu.regs().rdx;
    let csidl = emu.regs().r8 as i32;

    log_red!(emu, "shell32!SHGetSpecialFolderPathA csidl: 0x{:x}", csidl);

    match csidl_to_path(csidl) {
        Some(path) => {
            if psz_path != 0 {
                emu.maps.write_string(psz_path, path);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // TRUE
        }
        None => {
            emu.regs_mut().rax = 42; // FALSE
        }
    }
}
