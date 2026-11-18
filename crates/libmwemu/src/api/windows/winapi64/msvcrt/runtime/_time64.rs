use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _time64(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let t_ptr = api_try!(emu, call.arg(emu, 0));
    let now = 0x6718_0000u64;
    if t_ptr != 0 {
        emu.maps.write_qword(t_ptr, now);
    }
    log_red!(emu, "msvcrt!_time64 => 0x{:x}", now);
    emu.set_last_error(0);
    call.set_return(emu, now);
}
