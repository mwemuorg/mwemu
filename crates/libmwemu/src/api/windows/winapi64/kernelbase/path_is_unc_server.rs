use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsUNCServerA(emu: &mut emu::Emu) {
    path_is_unc_server(emu, "PathIsUNCServerA", false);
}

fn path_is_unc_server(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_server = path.starts_with("\\\\") && !path[2..].contains('\\');
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_server);
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_server { 1 } else { 0 };
}

pub fn PathIsUNCServerW(emu: &mut emu::Emu) {
    path_is_unc_server(emu, "PathIsUNCServerW", true);
}
