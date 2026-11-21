use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ScrollWindowEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let dx = api_try!(emu, call.arg_i32(emu, 1));
    let dy = api_try!(emu, call.arg_i32(emu, 2));
    let _lp_scroll = api_try!(emu, call.arg(emu, 3));
    let _lp_clip = api_try!(emu, call.arg(emu, 4));
    let hrgn_update = api_try!(emu, call.arg(emu, 5));
    let lp_rc_update = api_try!(emu, call.arg(emu, 6));
    let _flags = api_try!(emu, call.arg(emu, 7));
    if lp_rc_update != 0 {
        emu.maps.write_dword(lp_rc_update, 0);
        emu.maps.write_dword(lp_rc_update + 4, 0);
        emu.maps.write_dword(lp_rc_update + 8, 0);
        emu.maps.write_dword(lp_rc_update + 12, 0);
    }
    log_red!(
        emu,
        "user32!ScrollWindowEx hwnd:0x{:x} dx:{} dy:{} hrgn:0x{:x}",
        hwnd,
        dx,
        dy,
        hrgn_update
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // SIMPLEREGION
}
