use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn HPALETTE_UserSize64(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a0 = api_try!(emu, call.arg_u32(emu, 0));
    let _a1 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "ole32!HPALETTE_UserSize64 a0:0x{:x} a1:0x{:x}",
        _a0,
        _a1
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
