use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::winapi32;
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "LdrLoadDll" => LdrLoadDll(emu),
        "LdrLoadDll_gul" => LdrLoadDll_gul(emu),
        "LdrGetDllHandleEx" => LdrGetDllHandleEx(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn LdrLoadDll(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let libname_ptr = api_try!(emu, call.arg(emu, 3));
    let libaddr_ptr = api_try!(emu, call.arg(emu, 4));

    let libname = emu.maps.read_wide_string(libname_ptr);
    log_red!(emu, "ntdll!LdrLoadDll   lib: {}", libname);

    let base = winapi32::kernel32::load_library(emu, &libname);
    if base != 0 {
        emu.maps.write_dword(libaddr_ptr, base as u32);
    }

    call.set_return(emu, STATUS_SUCCESS);
}

fn LdrLoadDll_gul(emu: &mut emu::Emu) {
    LdrLoadDll(emu);
}

fn LdrGetDllHandleEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let dll_name_ptr = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let out_handle = api_try!(emu, call.arg_u32(emu, 3)) as u64;

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
    call.set_return(emu, 0xC0000135);
    call.cleanup_args(emu, 4);
}
