use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetTapePosition(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hDevice = api_try!(emu, call.arg_u32(emu, 0));
    let _dwPositionMethod = api_try!(emu, call.arg_u32(emu, 1));
    let _dwPartition = api_try!(emu, call.arg_u32(emu, 2));
    let _dwOffsetLow = api_try!(emu, call.arg_u32(emu, 3));
    let _dwOffsetHigh = api_try!(emu, call.arg_u32(emu, 4));
    let _bImmediate = api_try!(emu, call.arg_u32(emu, 5));

    log_red!(emu, "kernel32!SetTapePosition");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}
