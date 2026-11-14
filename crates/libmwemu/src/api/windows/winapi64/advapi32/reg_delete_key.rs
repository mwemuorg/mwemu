use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteKeyA(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyA", false);
}

fn reg_delete_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, subkey);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteKeyW(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyW", true);
}
