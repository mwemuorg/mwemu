use crate::emu;

pub fn PathRemoveFileSpecA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!PathRemoveFileSpecA path: '{}'", path);
    if let Some(pos) = path.rfind('\\').or_else(|| path.rfind('/')) {
        emu.maps.write_byte(path_ptr + pos as u64, 0);
        emu.regs_mut().rax = 1; // TRUE, spec removed
    } else {
        emu.regs_mut().rax = 0; // FALSE
    }
}
