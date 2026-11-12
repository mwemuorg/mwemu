use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExA", false);
}

fn reg_create_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2) as u32;
    let class_ptr = WinApiAbi::arg(emu, 3) as u32;
    let options = WinApiAbi::arg(emu, 4) as u32;
    let sam = WinApiAbi::arg(emu, 5) as u32;
    let security = WinApiAbi::arg(emu, 6) as u32;
    let result_ptr = WinApiAbi::arg(emu, 7);
    let disposition_ptr = WinApiAbi::arg(emu, 8);
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let new_handle = helper::handler_create(&subkey);
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, new_handle as u32);
    }
    if disposition_ptr != 0 {
        emu.maps.write_dword(disposition_ptr, 2);
    }
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        new_handle
    );
    for _ in 0..9 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExW", true);
}
