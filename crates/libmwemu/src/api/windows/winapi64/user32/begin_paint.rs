use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn BeginPaint(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let ps_ptr = api_try!(emu, call.arg(emu, 1));
    let hdc = helper::handler_create("hdc://paint");
    if ps_ptr != 0 {
        // PAINTSTRUCT: hdc(8) + fErase(4) + rcPaint(16) + ...
        emu.maps.write_qword(ps_ptr, hdc);
        emu.maps.write_dword(ps_ptr + 8, 0); // fErase = FALSE
        // rcPaint: left, top, right, bottom
        emu.maps.write_dword(ps_ptr + 12, 0);
        emu.maps.write_dword(ps_ptr + 16, 0);
        emu.maps.write_dword(ps_ptr + 20, 1920);
        emu.maps.write_dword(ps_ptr + 24, 1080);
    }
    log_red!(
        emu,
        "user32!BeginPaint hwnd:0x{:x} => hdc:0x{:x}",
        hwnd,
        hdc
    );
    call.set_return(emu, hdc);
}
