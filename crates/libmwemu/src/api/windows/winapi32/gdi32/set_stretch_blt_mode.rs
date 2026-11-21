use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetStretchBltMode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let mode = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "gdi32!SetStretchBltMode mode:{}", mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1); // previous mode
    call.cleanup_args(emu, 2);
}
