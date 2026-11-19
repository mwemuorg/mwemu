use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _time32(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let timer_ptr = api_try!(emu, call.arg(emu, 0));
    let time_val = emu.tick as u32;
    if timer_ptr != 0 {
        emu.maps.write_dword(timer_ptr, time_val);
    }
    log_red!(emu, "msvcrt!_time32 => {}", time_val);
    emu.set_last_error(0);
    call.set_return(emu, time_val as u64);
}
