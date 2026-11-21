use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpszShortPath = api_try!(emu, call.arg_u32(emu, 0));
    let lpszLongPath = api_try!(emu, call.arg_u32(emu, 1));
    let _cchBuffer = api_try!(emu, call.arg_u32(emu, 2));

    let lpszShortPath = emu.maps.read_string(lpszShortPath as u64);
    let lpszLongPath = emu.maps.read_string(lpszLongPath as u64);

    log_red!(
        emu,
        "kernel32!GetLongPathNameA {} {}",
        lpszShortPath,
        lpszLongPath
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let short_path_ptr = api_try!(emu, call.arg(emu, 0));
    let long_path_ptr = api_try!(emu, call.arg(emu, 1));
    let buff = api_try!(emu, call.arg(emu, 2));

    let short = emu.maps.read_wide_string(short_path_ptr);

    log_red!(
        emu,
        "kernel32!GetLongPathNameW  {} {:x}",
        short,
        long_path_ptr
    );

    if long_path_ptr > 0 {
        let mut base = String::from("\\.\\");
        base.push_str(&short);
        emu.maps.write_wide_string(long_path_ptr, &base);
    }

    call.set_return(emu, short.len() as u64);
    call.cleanup_args(emu, 3);
}
