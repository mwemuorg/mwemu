use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_wcsicmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1 = api_try!(emu, call.arg(emu, 0));
    let s2 = api_try!(emu, call.arg(emu, 1));
    let w1 = emu.maps.read_wide_string(s1).to_ascii_lowercase();
    let w2 = emu.maps.read_wide_string(s2).to_ascii_lowercase();
    let res = w1.cmp(&w2) as i32;
    log_red!(emu, "wincrt!_wcsicmp '{}' '{}' => {}", w1, w2, res);
    call.set_return(emu, res as u64);
}
