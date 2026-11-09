use crate::api::windows::winapi32;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "LdrLoadDll" => LdrLoadDll(emu),
        "LdrLoadDll_gul" => LdrLoadDll_gul(emu),
        "LdrGetDllHandleEx" => LdrGetDllHandleEx(emu),
        _ => return false,
    }
    true
}

fn LdrLoadDll(emu: &mut emu::Emu) {
    let libname_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let libaddr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let libname = emu.maps.read_wide_string(libname_ptr);
    log_red!(emu, "ntdll!LdrLoadDll   lib: {}", libname);

    let base = winapi32::kernel32::load_library(emu, &libname);
    if base != 0 {
        emu.maps.write_dword(libaddr_ptr, base as u32);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

fn LdrLoadDll_gul(emu: &mut emu::Emu) {
    LdrLoadDll(emu);
}

fn LdrGetDllHandleEx(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let dll_name_ptr = arg(emu, 2) as u64;
    let out_handle = arg(emu, 3) as u64;

    let dll_name = if dll_name_ptr != 0 {
        emu.maps.read_wide_string(dll_name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "ntdll!LdrGetDllHandleEx flags:{} dll:'{}'",
        flags,
        dll_name
    );

    if out_handle != 0 {
        emu.maps.write_dword(out_handle, 0);
    }
    emu.regs_mut().rax = 0xC0000135;
    stdcall_ret(emu, 4);
}
