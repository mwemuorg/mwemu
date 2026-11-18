use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetLayout(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let layout = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "gdi32!SetLayout layout:0x{:x}", layout);
    emu.set_last_error(0);
    call.set_return(emu, 0); // previous layout
    call.cleanup_args(emu, 2);
}
