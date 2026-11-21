use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn strdup(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let src_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(src_ptr);
    let len = s.len() as u64 + 1;
    let dst = allocate_memory(emu, len).unwrap_or(0);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "msvcrt!{} '{}' => 0x{:x}", api, s, dst);
    emu.set_last_error(0);
    call.set_return(emu, dst);
}
