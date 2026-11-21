use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wcsicmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1_ptr = api_try!(emu, call.arg(emu, 0));
    let s2_ptr = api_try!(emu, call.arg(emu, 1));
    let s1 = emu.maps.read_wide_string(s1_ptr).to_lowercase();
    let s2 = emu.maps.read_wide_string(s2_ptr).to_lowercase();
    let result = s1.cmp(&s2) as i32;
    log_red!(emu, "msvcrt!_wcsicmp '{}' '{}' => {}", s1, s2, result);
    emu.set_last_error(0);
    call.set_return(emu, result as u32 as u64);
}
