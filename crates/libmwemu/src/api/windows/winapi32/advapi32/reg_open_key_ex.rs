use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let subkey = emu
        .maps
        .read_string(api_try!(emu, call.arg_u32(emu, 1)) as u64);
    let result_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);
    if result_ptr != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg(emu, 2));
    let sam = api_try!(emu, call.arg(emu, 3));
    let result_ptr = api_try!(emu, call.arg(emu, 4));
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
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}
