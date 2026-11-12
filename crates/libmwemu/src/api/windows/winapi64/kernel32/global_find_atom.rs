use crate::emu;
use crate::winapi::common::strings;

pub fn GlobalFindAtomA(emu: &mut emu::Emu) {
    global_find_atom(emu, "GlobalFindAtomA", false);
}

fn global_find_atom(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name = emu.regs().rcx;
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}' => 0", name_s);
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
}

pub fn GlobalFindAtomW(emu: &mut emu::Emu) {
    global_find_atom(emu, "GlobalFindAtomW", true);
}
