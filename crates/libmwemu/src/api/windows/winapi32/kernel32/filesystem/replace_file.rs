use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn ReplaceFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!ReplaceFile h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    call.cleanup_args(emu, 2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn ReplaceFileA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpReplacedFileName = api_try!(emu, call.arg_u32(emu, 0));
    let lpReplacementFileName = api_try!(emu, call.arg_u32(emu, 1));
    let lpBackupFileName = api_try!(emu, call.arg_u32(emu, 2));
    let _dwReplaceFlags = api_try!(emu, call.arg_u32(emu, 3));
    let _lpExclude = api_try!(emu, call.arg_u32(emu, 4));
    let _lpReserved = api_try!(emu, call.arg_u32(emu, 5));

    let lpReplacedFileName = emu.maps.read_string(lpReplacedFileName as u64);
    let lpReplacementFileName = emu.maps.read_string(lpReplacementFileName as u64);
    let lpBackupFileName = emu.maps.read_string(lpBackupFileName as u64);

    log_red!(
        emu,
        "kernel32!ReplaceFileA {} {} {}",
        lpReplacedFileName,
        lpReplacementFileName,
        lpBackupFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn ReplaceFileW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!ReplaceFileW '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 6);
    call.set_return(emu, 1);
}
