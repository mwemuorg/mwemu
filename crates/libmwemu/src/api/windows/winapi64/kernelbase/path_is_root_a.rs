use crate::emu;

pub fn PathIsRootA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let is_root = path == "\\"
        || (path.len() == 3 && path.as_bytes()[1] == b':' && path.as_bytes()[2] == b'\\');
    log_red!(emu, "kernelbase!PathIsRootA '{}' => {}", path, is_root);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_root { 1 } else { 0 };
}
