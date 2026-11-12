use crate::emu;
use crate::winapi::common::strings;

pub fn DeleteFile2A(emu: &mut emu::Emu) {
    delete_file2(emu, "DeleteFile2A", false);
}

fn delete_file2(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = strings::read_text(emu, path_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn DeleteFile2W(emu: &mut emu::Emu) {
    delete_file2(emu, "DeleteFile2W", true);
}
