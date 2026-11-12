use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let name_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2) as u32;
    let value_type = WinApiAbi::arg(emu, 3) as u32;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_size = WinApiAbi::arg(emu, 5) as u32;
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
