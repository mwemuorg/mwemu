use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WriteTapemark(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hDevice = api_try!(emu, call.arg_u32(emu, 0));
    let _dwTapemarkType = api_try!(emu, call.arg_u32(emu, 1));
    let _dwTapemarkCount = api_try!(emu, call.arg_u32(emu, 2));
    let _bImmediate = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!WriteTapemark");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}
