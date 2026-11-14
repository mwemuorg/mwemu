use crate::emu;
use crate::winapi::common::strings;

pub fn RemoveDirectory2A(emu: &mut emu::Emu) {
    remove_directory2(emu, "RemoveDirectory2A", false);
}

fn remove_directory2(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = strings::read_text(emu, path_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn RemoveDirectory2W(emu: &mut emu::Emu) {
    remove_directory2(emu, "RemoveDirectory2W", true);
}
