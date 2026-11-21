use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn StrCmpCA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let s1_ptr = api_try!(emu, call.arg(emu, 0));
    let s2_ptr = api_try!(emu, call.arg(emu, 1));
    let s1 = emu.maps.read_string(s1_ptr);
    let s2 = emu.maps.read_string(s2_ptr);
    let result = s1.cmp(&s2) as i32;
    log_red!(emu, "wincrt!StrCmpCA '{}' '{}' => {}", s1, s2, result);
    call.set_return(emu, result as u32 as u64);
}
