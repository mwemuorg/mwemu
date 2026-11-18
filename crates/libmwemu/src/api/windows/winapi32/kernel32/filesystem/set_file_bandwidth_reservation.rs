use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetFileBandwidthReservation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hFile = api_try!(emu, call.arg_u32(emu, 0));
    let _nPeriodMilliseconds = api_try!(emu, call.arg_u32(emu, 1));
    let _nBytesPerPeriod = api_try!(emu, call.arg_u32(emu, 2));
    let _bDiscardable = api_try!(emu, call.arg_u32(emu, 3));
    let _lpTransferSize = api_try!(emu, call.arg_u32(emu, 4));
    let _lpNumOutstandingRequests = api_try!(emu, call.arg_u32(emu, 5));

    log_red!(emu, "kernel32!SetFileBandwidthReservation");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}
