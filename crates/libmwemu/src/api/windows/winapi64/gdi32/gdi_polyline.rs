use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GdiPolyline(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let points = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "gdi32!Polyline hdc:0x{:x} pts:0x{:x} count:{}",
        hdc,
        points,
        count
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
