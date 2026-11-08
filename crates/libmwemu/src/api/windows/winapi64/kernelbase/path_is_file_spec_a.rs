use crate::emu;

pub fn PathIsFileSpecA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        emu.maps.read_string(path_ptr)
    } else {
        String::new()
    };
    let is_filespec = !path.contains('\\') && !path.contains('/') && !path.contains(':');
    log_red!(
        emu,
        "kernelbase!PathIsFileSpecA '{}' => {}",
        path,
        is_filespec
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_filespec { 1 } else { 0 };
}
