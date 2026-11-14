use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteKeyExA(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExA", false);
}

fn reg_delete_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteKeyExW(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExW", true);
}
