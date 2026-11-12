use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteTreeA(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeA", false);
}

fn reg_delete_tree(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteTreeW(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeW", true);
}
