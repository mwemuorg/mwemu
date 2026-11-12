use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub(super) fn PathFileExistsA(emu: &mut emu::Emu) {
    path_file_exists(emu, "PathFileExistsA", false);
}

fn path_file_exists(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ptr_path = WinApiAbi::arg(emu, 0);

    let path = strings::read_text(emu, ptr_path, wide);

    log_red!(emu, "shlwapi!{api} path: {}", path);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // True
}

pub(super) fn PathFileExistsW(emu: &mut emu::Emu) {
    path_file_exists(emu, "PathFileExistsW", true);
}
