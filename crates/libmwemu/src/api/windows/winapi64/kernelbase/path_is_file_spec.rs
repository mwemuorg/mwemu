use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsFileSpecA(emu: &mut emu::Emu) {
    path_is_file_spec(emu, "PathIsFileSpecA", false);
}

fn path_is_file_spec(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_filespec = !path.contains('\\') && !path.contains('/') && !path.contains(':');
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_filespec);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_filespec { 1 } else { 0 };
}

pub fn PathIsFileSpecW(emu: &mut emu::Emu) {
    path_is_file_spec(emu, "PathIsFileSpecW", true);
}
