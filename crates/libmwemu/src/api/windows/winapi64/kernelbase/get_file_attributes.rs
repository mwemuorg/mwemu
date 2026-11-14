use crate::emu;
use crate::winapi::common::strings;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesA", false);
}

fn get_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = strings::read_text(emu, path_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80;
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesW", true);
}
