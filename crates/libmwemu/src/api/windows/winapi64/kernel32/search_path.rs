use crate::emu;
use crate::winapi::common::strings;

pub fn SearchPathA(emu: &mut emu::Emu) {
    search_path(emu, "SearchPathA", false);
}

fn search_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _path = emu.regs().rcx;
    let filename = emu.regs().rdx;
    let _ext = emu.regs().r8;
    let name_s = if filename != 0 {
        strings::read_text(emu, filename, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}' => 0 (not found)", name_s);
    emu.set_last_error(2); // ERROR_FILE_NOT_FOUND
    emu.regs_mut().rax = 0;
}

pub fn SearchPathW(emu: &mut emu::Emu) {
    search_path(emu, "SearchPathW", true);
}
