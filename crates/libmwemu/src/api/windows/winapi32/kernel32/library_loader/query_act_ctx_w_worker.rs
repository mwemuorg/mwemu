use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn QueryActCtxWWorker(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _int = api_try!(emu, call.arg_u32(emu, 0));
    let _Context = api_try!(emu, call.arg_u32(emu, 1));
    let _pvSubInstance = api_try!(emu, call.arg_u32(emu, 2));
    let _ulInfoClass = api_try!(emu, call.arg_u32(emu, 3));
    let _pvBuffer = api_try!(emu, call.arg_u32(emu, 4));
    let _cbBuffer = api_try!(emu, call.arg_u32(emu, 5));
    let _pcbWrittenOrRequired = api_try!(emu, call.arg_u32(emu, 6));

    log_red!(emu, "kernel32!QueryActCtxWWorker");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 7);
}
