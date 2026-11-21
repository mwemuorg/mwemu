use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BasepSxsCreateStreams(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));
    let _a2 = api_try!(emu, call.arg_u32(emu, 1));
    let _a3 = api_try!(emu, call.arg_u32(emu, 2));
    let _a4 = api_try!(emu, call.arg_u32(emu, 3));
    let _a5 = api_try!(emu, call.arg_u32(emu, 4));
    let _a6 = api_try!(emu, call.arg_u32(emu, 5));
    let _a7 = api_try!(emu, call.arg_u32(emu, 6));
    let _FileHandle = api_try!(emu, call.arg_u32(emu, 7));
    let _a9 = api_try!(emu, call.arg_u32(emu, 8));
    let _a10 = api_try!(emu, call.arg_u32(emu, 9));
    let _a11 = api_try!(emu, call.arg_u32(emu, 10));
    let _a12 = api_try!(emu, call.arg_u32(emu, 11));
    let _a13 = api_try!(emu, call.arg_u32(emu, 12));
    let _a14 = api_try!(emu, call.arg_u32(emu, 13));
    let _a15 = api_try!(emu, call.arg_u32(emu, 14));
    let _a16 = api_try!(emu, call.arg_u32(emu, 15));

    log_red!(emu, "kernel32!BasepSxsCreateStreams");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 16);
}
