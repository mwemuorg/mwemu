use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetCursorPos(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pt = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    log_red!(emu, "user32!GetCursorPos");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
