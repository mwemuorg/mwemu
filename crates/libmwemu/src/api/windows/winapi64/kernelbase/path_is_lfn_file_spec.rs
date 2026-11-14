use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsLFNFileSpecA(emu: &mut emu::Emu) {
    path_is_lfn_file_spec(emu, "PathIsLFNFileSpecA", false);
}

fn path_is_lfn_file_spec(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let filename = path
        .rfind('\\')
        .or_else(|| path.rfind('/'))
        .map(|p| &path[p + 1..])
        .unwrap_or(&path);
    let is_lfn = filename.len() > 12
        || filename.contains(' ')
        || filename.chars().filter(|c| *c == '.').count() > 1;
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_lfn);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_lfn { 1 } else { 0 };
}

pub fn PathIsLFNFileSpecW(emu: &mut emu::Emu) {
    path_is_lfn_file_spec(emu, "PathIsLFNFileSpecW", true);
}
