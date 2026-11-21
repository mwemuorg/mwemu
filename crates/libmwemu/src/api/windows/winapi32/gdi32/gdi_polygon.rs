use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GdiPolygon(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "gdi32!Polygon");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
