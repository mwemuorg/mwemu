use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strstr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let haystack = api_try!(emu, call.arg(emu, 0));
    let needle = api_try!(emu, call.arg(emu, 1));
    let h = emu.maps.read_string(haystack);
    let n = emu.maps.read_string(needle);
    let result = match h.find(&n) {
        Some(pos) => haystack + pos as u64,
        None => 0,
    };
    log_red!(emu, "wincrt!strstr '{}' '{}' => 0x{:x}", h, n, result);
    call.set_return(emu, result);
}
