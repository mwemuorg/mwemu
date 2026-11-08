use crate::emu;

pub fn PathRemoveFileSpecW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!PathRemoveFileSpecW path: '{}'", path);
    if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
        emu.maps.write_word(path_ptr + (pos as u64) * 2, 0);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
