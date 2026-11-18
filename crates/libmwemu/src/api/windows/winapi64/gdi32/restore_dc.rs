use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RestoreDC(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let saved = api_try!(emu, call.arg_i32(emu, 1));
    log_red!(emu, "gdi32!RestoreDC hdc:0x{:x} saved:{}", hdc, saved);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
