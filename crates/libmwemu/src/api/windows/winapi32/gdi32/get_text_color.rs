use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTextColor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));

    let arg1 = api_try!(emu, call.arg(emu, 0));
    call.cleanup_args(emu, 1);
    log_red!(emu, "gdi32!GetTextColor arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    call.set_return(emu, 0); // black
    call.cleanup_args(emu, 1);
}
