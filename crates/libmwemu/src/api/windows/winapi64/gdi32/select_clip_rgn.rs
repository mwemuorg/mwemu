use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SelectClipRgn(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let hrgn = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "gdi32!SelectClipRgn hdc:0x{:x} hrgn:0x{:x}", hdc, hrgn);
    emu.set_last_error(0);
    call.set_return(emu, 1); // SIMPLEREGION
}
