use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsRootA(emu: &mut emu::Emu) {
    path_is_root(emu, "PathIsRootA", false);
}

fn path_is_root(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_root = path == "\\"
        || (path.len() == 3 && path.as_bytes()[1] == b':' && path.as_bytes()[2] == b'\\');
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_root);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_root { 1 } else { 0 };
}

pub fn PathIsRootW(emu: &mut emu::Emu) {
    path_is_root(emu, "PathIsRootW", true);
}
