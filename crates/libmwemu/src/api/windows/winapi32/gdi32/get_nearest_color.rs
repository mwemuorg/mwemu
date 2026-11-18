use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetNearestColor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let color = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "gdi32!GetNearestColor color:0x{:06x}", color);
    emu.set_last_error(0);
    call.set_return(emu, color as u64);
    call.cleanup_args(emu, 2);
}
