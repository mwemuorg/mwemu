use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn DeviceIoControlImplementation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hDevice = api_try!(emu, call.arg_u32(emu, 0));
    let _dwIoControlCode = api_try!(emu, call.arg_u32(emu, 1));
    let _lpInBuffer = api_try!(emu, call.arg_u32(emu, 2));
    let _nInBufferSize = api_try!(emu, call.arg_u32(emu, 3));
    let _lpOutBuffer = api_try!(emu, call.arg_u32(emu, 4));
    let _nOutBufferSize = api_try!(emu, call.arg_u32(emu, 5));
    let _lpBytesReturned = api_try!(emu, call.arg_u32(emu, 6));
    let _lpOverlapped = api_try!(emu, call.arg_u32(emu, 7));

    log_red!(emu, "kernel32!DeviceIoControlImplementation");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 8);
}
