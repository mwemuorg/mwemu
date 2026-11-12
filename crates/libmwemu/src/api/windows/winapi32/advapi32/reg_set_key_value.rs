use crate::emu;
use crate::winapi::common::strings;

pub fn RegSetKeyValueA(emu: &mut emu::Emu) {
    reg_set_key_value(emu, "RegSetKeyValueA", false);
}

fn reg_set_key_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
