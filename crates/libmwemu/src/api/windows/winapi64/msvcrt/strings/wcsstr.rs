use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn wcsstr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let haystack_ptr = api_try!(emu, call.arg(emu, 0));
    let needle_ptr = api_try!(emu, call.arg(emu, 1));
    let haystack = emu.maps.read_wide_string(haystack_ptr);
    let needle = emu.maps.read_wide_string(needle_ptr);
    let result = match haystack.find(&needle) {
        Some(pos) => haystack_ptr + (pos * 2) as u64,
        None => 0,
    };
    log_red!(
        emu,
        "msvcrt!wcsstr '{}' '{}' => 0x{:x}",
        haystack,
        needle,
        result
    );
    emu.set_last_error(0);
    call.set_return(emu, result);
}
