use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg(emu, 0));
    let left = api_try!(emu, call.arg_u32(emu, 1));
    let top = api_try!(emu, call.arg_u32(emu, 2));
    let right = api_try!(emu, call.arg_u32(emu, 3));
    let bottom = api_try!(emu, call.arg_u32(emu, 4));
    if rect != 0 {
        emu.maps.write_dword(rect, left);
        emu.maps.write_dword(rect + 4, top);
        emu.maps.write_dword(rect + 8, right);
        emu.maps.write_dword(rect + 12, bottom);
    }
    log_red!(
        emu,
        "user32!SetRect ({},{},{},{}) => 0x{:x}",
        left,
        top,
        right,
        bottom,
        rect
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
