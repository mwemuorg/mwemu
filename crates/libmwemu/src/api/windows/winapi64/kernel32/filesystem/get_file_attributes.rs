use crate::emu;
use crate::winapi::common::strings;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesA", false);
}

fn get_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let filename_ptr = emu.regs().rcx;
    let filename = strings::read_text(emu, filename_ptr, wide);

    log_red!(emu, "kernel32!{api} file: '{}'", filename);

    let attrs: u64 = if filename.ends_with('\\') || filename.ends_with('/') {
        0x10 // FILE_ATTRIBUTE_DIRECTORY
    } else {
        0x80 // FILE_ATTRIBUTE_NORMAL
    };

    emu.set_last_error(0);
    emu.regs_mut().rax = attrs;
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesW", true);
}
