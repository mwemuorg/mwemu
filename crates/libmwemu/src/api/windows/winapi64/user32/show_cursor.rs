use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn ShowCursor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let bshow = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!ShowCursor bShow:{}", bshow);
    call.set_return(emu, if bshow != 0 { 1 } else { 0u64.wrapping_sub(1) });
}
