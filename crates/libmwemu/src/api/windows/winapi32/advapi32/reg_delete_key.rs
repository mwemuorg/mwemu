use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteKeyA(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyA", false);
}

fn reg_delete_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, subkey);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteKeyW(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyW", true);
}
