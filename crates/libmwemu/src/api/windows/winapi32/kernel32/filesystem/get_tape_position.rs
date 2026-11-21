use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetTapePosition(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hDevice = api_try!(emu, call.arg_u32(emu, 0));
    let _dwPositionType = api_try!(emu, call.arg_u32(emu, 1));
    let _lpdwPartition = api_try!(emu, call.arg_u32(emu, 2));
    let _lpdwOffsetLow = api_try!(emu, call.arg_u32(emu, 3));
    let _lpdwOffsetHigh = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!GetTapePosition");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}
