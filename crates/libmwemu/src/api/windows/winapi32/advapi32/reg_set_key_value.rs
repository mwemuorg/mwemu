use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegSetKeyValueA(emu: &mut emu::Emu) {
    reg_set_key_value(emu, "RegSetKeyValueA", false);
}

fn reg_set_key_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2);
    let reg_type = WinApiAbi::arg(emu, 3);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.regs_mut().rax = 0;
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn RegSetKeyValueW(emu: &mut emu::Emu) {
    reg_set_key_value(emu, "RegSetKeyValueW", true);
}
