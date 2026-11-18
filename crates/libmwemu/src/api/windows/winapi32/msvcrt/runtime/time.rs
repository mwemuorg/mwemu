use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn time(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let time_ptr = api_try!(emu, call.arg(emu, 0));
    let tick = emu.tick as u32;
    if time_ptr != 0 {
        emu.maps.write_dword(time_ptr, tick);
    }
    log_red!(emu, "msvcrt!time => {}", tick);

    call.set_return(emu, tick as u64);
}
