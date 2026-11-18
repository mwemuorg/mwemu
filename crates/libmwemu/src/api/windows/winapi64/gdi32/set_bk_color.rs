use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetBkColor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let color = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "gdi32!SetBkColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0x00ff_ffff);
}
