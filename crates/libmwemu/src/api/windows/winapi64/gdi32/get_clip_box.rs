use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetClipBox(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let rect = api_try!(emu, call.arg(emu, 1));
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "gdi32!GetClipBox hdc:0x{:x} rect:0x{:x}", hdc, rect);
    call.set_return(emu, 1); // SIMPLEREGION
}
