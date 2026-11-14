use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExA", false);
}

fn reg_create_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hKey = WinApiAbi::arg(emu, 0);
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2);
    let class_ptr = WinApiAbi::arg(emu, 3);
    let options = WinApiAbi::arg(emu, 4);
    let security_attr = WinApiAbi::arg(emu, 5);

    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let mut class_name = "".to_string();
    if class_ptr > 0 {
        class_name = strings::read_text(emu, class_ptr, wide);
    }

    log_red!(emu, "kernel32!{api} {} {}", subkey, class_name);
    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..9 {
        emu.stack_pop32(false);
    }
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExW", true);
}
