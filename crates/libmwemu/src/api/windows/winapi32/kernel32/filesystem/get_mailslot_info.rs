use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetMailslotInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hMailslot = api_try!(emu, call.arg_u32(emu, 0));
    let _lpMaxMessageSize = api_try!(emu, call.arg_u32(emu, 1));
    let _lpNextSize = api_try!(emu, call.arg_u32(emu, 2));
    let _lpMessageCount = api_try!(emu, call.arg_u32(emu, 3));
    let _lpReadTimeout = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!GetMailslotInfo");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}
