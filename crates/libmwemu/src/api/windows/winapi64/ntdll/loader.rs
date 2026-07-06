use crate::api::windows::winapi64;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "LdrLoadDll" => LdrLoadDll(emu),
        "LdrGetDllHandleEx" => LdrGetDllHandleEx(emu),
        _ => return false,
    }
    true
}

fn LdrLoadDll(emu: &mut emu::Emu) {
    let libname_ptr = emu.regs().r8;
    let libaddr_ptr = emu.regs().r9;

    let libname = emu.maps.read_wide_string(libname_ptr);
    log_red!(emu, "ntdll!LdrLoadDll   lib: {}", libname);

    let base = winapi64::kernel32::load_library(emu, &libname);
    if base != 0 {
        emu.maps.write_qword(libaddr_ptr, base);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn LdrGetDllHandleEx(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let characteristics = emu.regs().r8;
    let dll_name_ptr = emu.regs().r9;
    let out_hndl = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!LdrGetDllHandleEx error reading out_hdl");

    let dll_name = emu.maps.read_wide_string(dll_name_ptr);

    log_red!(emu, "ntdll!LdrGetDllHandleEx {}", dll_name);

    let result = emu.maps.memcpy(path_ptr, dll_name_ptr, dll_name.len());
    if !result {
        panic!("LdrGetDllHandleEx failed to copy");
    }

    let handle = helper::handler_create(&dll_name);
    emu.maps.write_qword(out_hndl, handle);

    emu.regs_mut().rax = 1;
}
