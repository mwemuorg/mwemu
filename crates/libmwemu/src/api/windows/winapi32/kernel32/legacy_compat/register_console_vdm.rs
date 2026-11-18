use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegisterConsoleVDM(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));
    let _a2 = api_try!(emu, call.arg_u32(emu, 1));
    let _a3 = api_try!(emu, call.arg_u32(emu, 2));
    let _a4 = api_try!(emu, call.arg_u32(emu, 3));
    let _a5 = api_try!(emu, call.arg_u32(emu, 4));
    let _a6 = api_try!(emu, call.arg_u32(emu, 5));
    let _a7 = api_try!(emu, call.arg_u32(emu, 6));
    let _a8 = api_try!(emu, call.arg_u32(emu, 7));
    let _a9 = api_try!(emu, call.arg_u32(emu, 8));

    log_red!(emu, "kernel32!RegisterConsoleVDM");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 9);
}
