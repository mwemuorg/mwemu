use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetClientRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1904);
        emu.maps.write_dword(rect + 12, 1041);
    }
    log_red!(emu, "user32!GetClientRect");
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
