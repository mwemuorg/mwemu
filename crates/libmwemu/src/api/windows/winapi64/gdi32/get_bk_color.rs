use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetBkColor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "gdi32!GetBkColor hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0x00ff_ffff);
}
