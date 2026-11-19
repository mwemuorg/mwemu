use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::winapi64;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "LdrLoadDll" | "LdrLoadDll_gul" => LdrLoadDll(emu),
        "LdrGetDllHandleEx" => LdrGetDllHandleEx(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn LdrLoadDll(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let libname_ptr = api_try!(emu, call.arg(emu, 2));
    let libaddr_ptr = api_try!(emu, call.arg(emu, 3));

    let libname = emu.maps.read_wide_string(libname_ptr);
    log_red!(emu, "ntdll!LdrLoadDll   lib: {}", libname);

    let base = winapi64::kernel32::load_library(emu, &libname);
    if base != 0 {
        emu.maps.write_qword(libaddr_ptr, base);
    }

    call.set_return(emu, STATUS_SUCCESS);
}

fn LdrGetDllHandleEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg(emu, 0));
    let path_ptr = api_try!(emu, call.arg(emu, 1));
    let characteristics = api_try!(emu, call.arg(emu, 2));
    let dll_name_ptr = api_try!(emu, call.arg(emu, 3));
    let out_hndl = api_try!(emu, call.arg(emu, 4));

    let dll_name = emu.maps.read_wide_string(dll_name_ptr);

    log_red!(emu, "ntdll!LdrGetDllHandleEx {}", dll_name);

    let result = emu.maps.memcpy(path_ptr, dll_name_ptr, dll_name.len());
    if !result {
        {
            log::warn!("LdrGetDllHandleEx failed to copy");
            return;
        }
    }

    let handle = helper::handler_create(&dll_name);
    emu.maps.write_qword(out_hndl, handle);

    call.set_return(emu, 1);
}
