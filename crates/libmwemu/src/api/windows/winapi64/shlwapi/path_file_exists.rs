use crate::emu;
use crate::winapi::common::strings;

pub(super) fn PathFileExistsA(emu: &mut emu::Emu) {
    path_file_exists(emu, "PathFileExistsA", false);
}

fn path_file_exists(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ptr_path = emu.regs().rcx;
    let path = strings::read_text(emu, ptr_path, wide);

    log_red!(emu, "shlwapi!{api} path: {}", path);

    emu.regs_mut().rax = 1; // True
}

pub(super) fn PathFileExistsW(emu: &mut emu::Emu) {
    path_file_exists(emu, "PathFileExistsW", true);
}
