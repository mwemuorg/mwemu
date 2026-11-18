use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn StringFromGUID2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a0 = api_try!(emu, call.arg_u32(emu, 0));
    let _a1 = api_try!(emu, call.arg_u32(emu, 1));
    let _a2 = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "ole32!StringFromGUID2 a0:0x{:x} a1:0x{:x}", _a0, _a1);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
