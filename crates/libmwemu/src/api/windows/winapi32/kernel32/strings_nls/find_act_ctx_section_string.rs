use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindActCtxSectionStringA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 0));
    let _lpExtensionGuid = api_try!(emu, call.arg_u32(emu, 1));
    let _ulSectionId = api_try!(emu, call.arg_u32(emu, 2));
    let lpStringToFind = api_try!(emu, call.arg_u32(emu, 3));
    let _ReturnedData = api_try!(emu, call.arg_u32(emu, 4));

    let lpStringToFind = emu.maps.read_string(lpStringToFind as u64);

    log_red!(emu, "kernel32!FindActCtxSectionStringA {}", lpStringToFind);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn FindActCtxSectionStringW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!FindActCtxSectionStringW '{}'  0x{:x}",
        s,
        arg2
    );
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}
