use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegDeleteKeyA(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyA", false);
}

fn reg_delete_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let subkey_ptr = WinApiAbi::arg(emu, 1);
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
