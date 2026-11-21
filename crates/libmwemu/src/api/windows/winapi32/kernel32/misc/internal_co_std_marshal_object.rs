use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn InternalCoStdMarshalObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _pStm = api_try!(emu, call.arg_u32(emu, 0));
    let _riid = api_try!(emu, call.arg_u32(emu, 1));
    let _pUnk = api_try!(emu, call.arg_u32(emu, 2));
    let _pServerCtx = api_try!(emu, call.arg_u32(emu, 3));
    let _dwDestCtx = api_try!(emu, call.arg_u32(emu, 4));
    let _pvDestCtx = api_try!(emu, call.arg_u32(emu, 5));
    let _mshlflags = api_try!(emu, call.arg_u32(emu, 6));

    log_red!(emu, "kernel32!InternalCoStdMarshalObject");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 7);
}
