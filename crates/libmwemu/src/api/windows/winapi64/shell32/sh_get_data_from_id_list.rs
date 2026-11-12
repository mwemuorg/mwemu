use crate::emu;
use crate::winapi::common::strings;

pub fn SHGetDataFromIDListA(emu: &mut emu::Emu) {
    sh_get_data_from_id_list(emu, "SHGetDataFromIDListA", false);
}

fn sh_get_data_from_id_list(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "shell32!{api} '{}'  0x{:x}", s, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHGetDataFromIDListW(emu: &mut emu::Emu) {
    sh_get_data_from_id_list(emu, "SHGetDataFromIDListW", true);
}
