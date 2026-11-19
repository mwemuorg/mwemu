use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn _tlgWriteTransfer_EventWriteTransfer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hProvider = api_try!(emu, call.arg_u32(emu, 0));
    let _pEventMetadata = api_try!(emu, call.arg_u32(emu, 1));
    let _pActivityId = api_try!(emu, call.arg_u32(emu, 2));
    let _pRelatedActivityId = api_try!(emu, call.arg_u32(emu, 3));
    let _cData = api_try!(emu, call.arg_u32(emu, 4));
    let _pData = api_try!(emu, call.arg_u32(emu, 5));

    log_red!(emu, "kernel32!_tlgWriteTransfer_EventWriteTransfer");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}
