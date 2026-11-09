use crate::emu;

pub fn PathFindFileNameA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!PathFindFileNameA path: '{}'", path);
    if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
        emu.set_last_error(0);
        emu.regs_mut().rax = path_ptr + (pos as u64) + 1;
    } else {
        emu.regs_mut().rax = path_ptr;
    }
}
