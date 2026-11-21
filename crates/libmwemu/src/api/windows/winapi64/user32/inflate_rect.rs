use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InflateRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg(emu, 0));
    let dx = api_try!(emu, call.arg_i32(emu, 1));
    let dy = api_try!(emu, call.arg_i32(emu, 2));
    if rect != 0 {
        let left = emu.maps.read_dword(rect).unwrap_or(0) as i32 - dx;
        let top = emu.maps.read_dword(rect + 4).unwrap_or(0) as i32 - dy;
        let right = emu.maps.read_dword(rect + 8).unwrap_or(0) as i32 + dx;
        let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0) as i32 + dy;
        emu.maps.write_dword(rect, left as u32);
        emu.maps.write_dword(rect + 4, top as u32);
        emu.maps.write_dword(rect + 8, right as u32);
        emu.maps.write_dword(rect + 12, bottom as u32);
    }
    log_red!(
        emu,
        "user32!InflateRect rect:0x{:x} dx:{} dy:{}",
        rect,
        dx,
        dy
    );
    call.set_return(emu, 1);
}
