use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let value_type = emu.regs().r9;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_size = WinApiAbi::arg(emu, 5);
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
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
