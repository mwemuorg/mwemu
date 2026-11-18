use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetClientRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let window = api_try!(emu, call.arg(emu, 0));
    let rect = api_try!(emu, call.arg(emu, 1));
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1904);
        emu.maps.write_dword(rect + 12, 1041);
    }
    log_red!(
        emu,
        "user32!GetClientRect window:0x{:x} rect:0x{:x}",
        window,
        rect
    );
    call.set_return(emu, 1);
}
