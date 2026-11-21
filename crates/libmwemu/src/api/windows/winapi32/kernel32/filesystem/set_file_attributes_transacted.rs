use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetFileAttributesTransactedA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(str_ptr);
    log_red!(
        emu,
        "kernel32!SetFileAttributesTransactedA '{}'  0x{:x}",
        s,
        arg2
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}

pub fn SetFileAttributesTransactedW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpFileName = api_try!(emu, call.arg_u32(emu, 0));
    let _dwFileAttributes = api_try!(emu, call.arg_u32(emu, 1));
    let _hTransaction = api_try!(emu, call.arg_u32(emu, 2));

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!SetFileAttributesTransactedW {}", lpFileName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
