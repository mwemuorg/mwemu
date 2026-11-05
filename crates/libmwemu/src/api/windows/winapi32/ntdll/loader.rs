use crate::api::windows::winapi32;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

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

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn LdrLoadDll_gul(emu: &mut emu::Emu) {
    LdrLoadDll(emu);
}

fn LdrGetDllHandleEx(emu: &mut emu::Emu) {
    let _flags = arg(emu, 0);
    let _path_ptr = arg(emu, 1) as u64;
    let _characteristics = arg(emu, 2);
    let dll_name_ptr = arg(emu, 3) as u64;
    let out_hndl = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let dll_name = emu.maps.read_wide_string(dll_name_ptr);

    log_red!(emu, "ntdll!LdrGetDllHandleEx {}", dll_name);

    let handle = helper::handler_create(&dll_name);
    assert!(handle < 0x00000001_00000000);
    emu.maps.write_dword(out_hndl, handle as u32);

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
}
