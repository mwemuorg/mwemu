use crate::emu;

pub fn PathRemoveBackslashA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    if path.ends_with('\\') {
        let end_pos = path.len() - 1;
        emu.maps.write_byte(path_ptr + end_pos as u64, 0);
        emu.regs_mut().rax = path_ptr + end_pos as u64;
    } else {
        let end_pos = path.len();
        emu.regs_mut().rax = path_ptr + end_pos as u64;
    }
    log_red!(emu, "kernelbase!PathRemoveBackslashA '{}'", path);
}
