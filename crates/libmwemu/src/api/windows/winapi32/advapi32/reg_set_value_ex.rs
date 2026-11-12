use crate::emu;
use crate::winapi::common::strings;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let value_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_size = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let name = strings::read_text(emu, name_ptr, wide);
    let data_str = if value_type == 1 || value_type == 2 {
        strings::read_text(emu, data_ptr, wide)
    } else {
        format!("binary[{}]", data_size)
    };
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' type:{} data:'{}'",
        hkey,
        name,
        value_type,
        data_str
    );
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
