use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetRectEmpty(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg(emu, 0));
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 0);
        emu.maps.write_dword(rect + 12, 0);
    }
    log_red!(emu, "user32!SetRectEmpty 0x{:x}", rect);
    call.set_return(emu, 1);
}
