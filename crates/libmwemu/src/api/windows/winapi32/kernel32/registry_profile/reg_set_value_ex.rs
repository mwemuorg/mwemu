use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hKey = WinApiAbi::arg(emu, 0);
    let value_name_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2);
    let value_type = WinApiAbi::arg(emu, 3);
    let data_ptr = WinApiAbi::arg(emu, 4);
    let data_size = WinApiAbi::arg(emu, 5);

    let value_name = strings::read_text(emu, value_name_ptr, wide);

    log_red!(
        emu,
        "kernel32!{api} `{}` type: {} data: 0x{:x}",
        value_name,
        value_type,
        data_ptr
    );
    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
