use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_time(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let t_ptr = api_try!(emu, call.arg(emu, 0));
    let now = 0x6718_0000u64;
    if t_ptr != 0 {
        emu.maps.write_qword(t_ptr, now);
    }
    log_red!(emu, "wincrt!time => 0x{:x}", now);
    call.set_return(emu, now);
}
