use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetInformationJobObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hJob = api_try!(emu, call.arg_u32(emu, 0));
    let _JobObjectInformationClass = api_try!(emu, call.arg_u32(emu, 1));
    let _lpJobObjectInformation = api_try!(emu, call.arg_u32(emu, 2));
    let _cbJobObjectInformationLength = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!SetInformationJobObject");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}
