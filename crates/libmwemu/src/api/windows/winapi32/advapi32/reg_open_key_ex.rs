use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let subkey = emu.maps.read_string(arg(emu, 1) as u64);
    let result_ptr = arg(emu, 4) as u64;
    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);
    if result_ptr != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 5);
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0);
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2);
    let sam = WinApiAbi::arg(emu, 3);
    let result_ptr = WinApiAbi::arg(emu, 4);
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!RegOpenKeyExW hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.regs_mut().rax = 0;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
