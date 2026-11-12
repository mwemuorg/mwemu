use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExA", false);
}

fn reg_create_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hKey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let options = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let security_attr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;

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
