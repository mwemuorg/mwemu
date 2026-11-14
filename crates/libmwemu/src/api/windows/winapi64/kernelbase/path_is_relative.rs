use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsRelativeA(emu: &mut emu::Emu) {
    path_is_relative(emu, "PathIsRelativeA", false);
}

fn path_is_relative(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = strings::read_text(emu, path_ptr, wide);
    let is_relative = !path.starts_with("\\") && !path.contains(":\\") && !path.starts_with("/");
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_relative);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_relative { 1 } else { 0 };
}

pub fn PathIsRelativeW(emu: &mut emu::Emu) {
    path_is_relative(emu, "PathIsRelativeW", true);
}
