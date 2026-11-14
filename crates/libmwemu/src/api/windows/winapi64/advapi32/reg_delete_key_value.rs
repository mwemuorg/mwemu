use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteKeyValueA(emu: &mut emu::Emu) {
    reg_delete_key_value(emu, "RegDeleteKeyValueA", false);
}

fn reg_delete_key_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteKeyValueW(emu: &mut emu::Emu) {
    reg_delete_key_value(emu, "RegDeleteKeyValueW", true);
}
