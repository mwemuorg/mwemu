use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetLayout(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "gdi32!GetLayout hdc:0x{:x}", hdc);
    emu.set_last_error(0);
    call.set_return(emu, 0); // LAYOUT_LTR (default left-to-right)
}
