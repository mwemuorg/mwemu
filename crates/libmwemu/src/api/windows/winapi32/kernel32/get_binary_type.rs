use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetBinaryType(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!GetBinaryType h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    call.cleanup_args(emu, 2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn GetBinaryTypeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(str_ptr);
    log_red!(emu, "kernel32!GetBinaryTypeA '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}

pub fn GetBinaryTypeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpApplicationName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpBinaryType = api_try!(emu, call.arg_u32(emu, 1));

    let lpApplicationName = emu.maps.read_wide_string(lpApplicationName as u64);

    log_red!(emu, "kernel32!GetBinaryTypeW {}", lpApplicationName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
