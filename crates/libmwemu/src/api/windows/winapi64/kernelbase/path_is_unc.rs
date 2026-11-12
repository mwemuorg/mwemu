use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsUNCA(emu: &mut emu::Emu) {
    path_is_unc(emu, "PathIsUNCA", false);
}

fn path_is_unc(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_unc = path.starts_with("\\\\");
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_unc);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_unc { 1 } else { 0 };
}

pub fn PathIsUNCW(emu: &mut emu::Emu) {
    path_is_unc(emu, "PathIsUNCW", true);
}
