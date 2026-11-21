use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RemoveDirectoryTransactedA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(str_ptr);
    log_red!(
        emu,
        "kernel32!RemoveDirectoryTransactedA '{}'  0x{:x}",
        s,
        arg2
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}

pub fn RemoveDirectoryTransactedW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpPathName = api_try!(emu, call.arg_u32(emu, 0));
    let _hTransaction = api_try!(emu, call.arg_u32(emu, 1));

    let lpPathName = emu.maps.read_wide_string(lpPathName as u64);

    log_red!(emu, "kernel32!RemoveDirectoryTransactedW {}", lpPathName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
