use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsRectEmpty(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg(emu, 0));
    let empty = if rect != 0 {
        let left = emu.maps.read_dword(rect).unwrap_or(0);
        let top = emu.maps.read_dword(rect + 4).unwrap_or(0);
        let right = emu.maps.read_dword(rect + 8).unwrap_or(0);
        let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0);
        right <= left || bottom <= top
    } else {
        true
    };
    log_red!(emu, "user32!IsRectEmpty 0x{:x} => {}", rect, empty);
    call.set_return(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, if empty { 1 } else { 0 });
}
