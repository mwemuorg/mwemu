use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _controlfp_s(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let p_current = api_try!(emu, call.arg(emu, 0));
    if p_current != 0 {
        emu.maps.write_dword(p_current, 0x0008_001f);
    }
    log_red!(emu, "msvcrt!_controlfp_s");
    call.set_return(emu, 0);
}
