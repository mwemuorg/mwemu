use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CopyFileExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpExistingFileName = api_try!(emu, call.arg_u32(emu, 0));
    let lpNewFileName = api_try!(emu, call.arg_u32(emu, 1));
    let _lpProgressRoutine = api_try!(emu, call.arg_u32(emu, 2));
    let _lpData = api_try!(emu, call.arg_u32(emu, 3));
    let _pbCancel = api_try!(emu, call.arg_u32(emu, 4));
    let _dwCopyFlags = api_try!(emu, call.arg_u32(emu, 5));

    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!CopyFileExA {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn CopyFileExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!CopyFileExW '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 6);
    call.set_return(emu, 1);
}
