use crate::emu;

pub fn PathFindExtensionA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let filename = path
        .rfind('\\')
        .or_else(|| path.rfind('/'))
        .map(|p| &path[p + 1..])
        .unwrap_or(&path);
    if let Some(dot_pos) = filename.rfind('.') {
        let abs_dot = path.len() - filename.len() + dot_pos;
        emu.set_last_error(0);
        emu.regs_mut().rax = path_ptr + abs_dot as u64;
    } else {
        emu.regs_mut().rax = path_ptr + path.len() as u64;
    }
    log_red!(emu, "kernelbase!PathFindExtensionA '{}'", path);
}
