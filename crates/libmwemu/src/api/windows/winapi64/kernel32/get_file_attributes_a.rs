use crate::emu;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let filename = emu.maps.read_string(filename_ptr);

    log_red!(emu, "kernel32!GetFileAttributesA file: '{}'", filename);

    let attrs: u64 = if filename.ends_with('\\') || filename.ends_with('/') {
        0x10 // FILE_ATTRIBUTE_DIRECTORY
    } else {
        0x80 // FILE_ATTRIBUTE_NORMAL
    };

    emu.set_last_error(0);
    emu.regs_mut().rax = attrs;
}
