use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_stricmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1 = api_try!(emu, call.arg(emu, 0));
    let s2 = api_try!(emu, call.arg(emu, 1));
    let str1 = emu.maps.read_string(s1).to_ascii_lowercase();
    let str2 = emu.maps.read_string(s2).to_ascii_lowercase();
    let res = str1.cmp(&str2) as i32;
    log_red!(emu, "wincrt!_stricmp '{}' '{}' => {}", str1, str2, res);
    call.set_return(emu, res as u64);
}
