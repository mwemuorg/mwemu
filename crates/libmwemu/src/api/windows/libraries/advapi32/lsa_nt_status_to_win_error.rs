use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;

pub fn LsaNtStatusToWinError(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let status = u64::from(api_try!(emu, call.arg_u32(emu, 0)));
    let value = match status {
        STATUS_SUCCESS => ERROR_SUCCESS,
        STATUS_INVALID_PARAMETER => ERROR_INVALID_PARAMETER,
        STATUS_ACCESS_DENIED => ERROR_ACCESS_DENIED,
        STATUS_OBJECT_NAME_NOT_FOUND => ERROR_FILE_NOT_FOUND,
        STATUS_NO_MEMORY => ERROR_NOT_ENOUGH_MEMORY,
        STATUS_INVALID_HANDLE => ERROR_INVALID_HANDLE,
        STATUS_BUFFER_TOO_SMALL => ERROR_INSUFFICIENT_BUFFER,
        STATUS_NOT_IMPLEMENTED => ERROR_INVALID_FUNCTION,
        _ => {
            policy::unsupported(emu, "advapi32", "LsaNtStatusToWinError", emu.pc());
            return;
        }
    };
    log_red!(
        emu,
        "advapi32!LsaNtStatusToWinError status={status:#x} error={value}"
    );
    call.set_return(emu, value);
    call.cleanup_args(emu, 1);
}
