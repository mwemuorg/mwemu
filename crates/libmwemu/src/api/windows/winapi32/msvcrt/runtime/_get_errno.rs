use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _get_errno(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let out_ptr = api_try!(emu, call.arg(emu, 0));
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    log_red!(emu, "msvcrt!_get_errno => 0");

    call.set_return(emu, 0);
}
