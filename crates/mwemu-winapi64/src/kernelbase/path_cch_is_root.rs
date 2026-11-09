use crate::emu;

pub fn PathCchIsRoot(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_wide_string(path_ptr);
    let is_root = path == "\\" || (path.len() == 3 && path.ends_with(":\\"));
    log_red!(emu, "kernelbase!PathCchIsRoot '{}' => {}", path, is_root);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_root { 1 } else { 0 };
}
