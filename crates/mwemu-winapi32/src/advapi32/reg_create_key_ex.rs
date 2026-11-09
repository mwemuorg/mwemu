use crate::emu;
use crate::winapi::helper;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let options = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let sam = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let security = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
    let disposition_ptr = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
    let subkey = emu.maps.read_string(subkey_ptr);
    let new_handle = helper::handler_create(&subkey);
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, new_handle as u32);
    }
    if disposition_ptr != 0 {
        emu.maps.write_dword(disposition_ptr, 2);
    }
    log_red!(
        emu,
        "advapi32!RegCreateKeyExA hkey:0x{:x} '{}' => 0x{:x}",
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
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let options = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let sam = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let security = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
    let disposition_ptr = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0) as u64;
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let new_handle = helper::handler_create(&subkey);
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, new_handle as u32);
    }
    if disposition_ptr != 0 {
        emu.maps.write_dword(disposition_ptr, 2);
    }
    log_red!(
        emu,
        "advapi32!RegCreateKeyExW hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        new_handle
    );
    for _ in 0..9 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
