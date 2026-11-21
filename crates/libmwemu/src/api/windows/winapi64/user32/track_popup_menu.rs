use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn TrackPopupMenu(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmenu = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let x = api_try!(emu, call.arg_i32(emu, 2));
    let y = api_try!(emu, call.arg_i32(emu, 3));
    let _reserved = api_try!(emu, call.arg(emu, 4));
    let hwnd = api_try!(emu, call.arg(emu, 5));
    let _lp_tpm = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "user32!TrackPopupMenu hmenu:0x{:x} flags:0x{:x} x:{} y:{} hwnd:0x{:x}",
        hmenu,
        flags,
        x,
        y,
        hwnd
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
