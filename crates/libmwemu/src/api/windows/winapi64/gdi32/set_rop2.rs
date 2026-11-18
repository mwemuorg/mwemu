use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetROP2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "gdi32!SetROP2 hdc:0x{:x} mode:{}", hdc, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 13); // previous: R2_COPYPEN
}
