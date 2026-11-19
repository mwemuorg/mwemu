use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetViewportOrgEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let x = api_try!(emu, call.arg_u32(emu, 1));
    let y = api_try!(emu, call.arg_u32(emu, 2));
    let prev = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    if prev != 0 {
        emu.maps.write_dword(prev, 0);
        emu.maps.write_dword(prev + 4, 0);
    }
    log_red!(emu, "gdi32!SetViewportOrgEx x:{} y:{}", x, y);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
