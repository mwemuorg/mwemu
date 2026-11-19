use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DragQueryPoint(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdrop = api_try!(emu, call.arg(emu, 0));
    let point_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "shell32!DragQueryPoint hdrop:0x{:x}", hdrop);
    if point_ptr != 0 {
        emu.maps.write_dword(point_ptr, 0);
        emu.maps.write_dword(point_ptr + 4, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
