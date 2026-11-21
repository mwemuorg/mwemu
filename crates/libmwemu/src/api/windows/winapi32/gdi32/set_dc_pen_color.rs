use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetDCPenColor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let color = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "gdi32!SetDCPenColor color:0x{:06x}", color);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0); // previous color
    call.cleanup_args(emu, 2);
}
