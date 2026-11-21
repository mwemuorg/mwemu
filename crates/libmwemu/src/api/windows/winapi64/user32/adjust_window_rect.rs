use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn AdjustWindowRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lprect = api_try!(emu, call.arg(emu, 0));
    let style = api_try!(emu, call.arg(emu, 1));
    let bmenu = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "user32!AdjustWindowRect rect:0x{:x} style:0x{:x} menu:{}",
        lprect,
        style,
        bmenu
    );
    if lprect != 0 && emu.maps.is_mapped(lprect) {
        let left = emu.maps.read_dword(lprect).unwrap_or(0) as i32 - 8;
        let top = emu.maps.read_dword(lprect + 4).unwrap_or(0) as i32 - 30;
        let right = emu.maps.read_dword(lprect + 8).unwrap_or(0) as i32 + 8;
        let bottom = emu.maps.read_dword(lprect + 12).unwrap_or(0) as i32 + 8;
        emu.maps.write_dword(lprect, left as u32);
        emu.maps.write_dword(lprect + 4, top as u32);
        emu.maps.write_dword(lprect + 8, right as u32);
        emu.maps.write_dword(lprect + 12, bottom as u32);
    }
    call.set_return(emu, 1);
}
