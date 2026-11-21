use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PtInRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg(emu, 0));
    let pt_x = api_try!(emu, call.arg_i32(emu, 1));
    let pt_y = api_try!(emu, call.arg_i32(emu, 2));
    let inside = if rect != 0 {
        let left = emu.maps.read_dword(rect).unwrap_or(0) as i32;
        let top = emu.maps.read_dword(rect + 4).unwrap_or(0) as i32;
        let right = emu.maps.read_dword(rect + 8).unwrap_or(0) as i32;
        let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0) as i32;
        pt_x >= left && pt_x < right && pt_y >= top && pt_y < bottom
    } else {
        false
    };
    log_red!(
        emu,
        "user32!PtInRect rect:0x{:x} pt:({},{}) => {}",
        rect,
        pt_x,
        pt_y,
        inside
    );
    emu.set_last_error(0);
    call.set_return(emu, if inside { 1 } else { 0 });
}
