use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetTextAlign(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let align = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "gdi32!SetTextAlign align:0x{:x}", align);
    emu.set_last_error(0);
    call.set_return(emu, 0); // previous alignment
    call.cleanup_args(emu, 2);
}
