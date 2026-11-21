use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn wcschr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u16(emu, 1));
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let s = emu.maps.read_wide_string(s_ptr);
    let result = match s.find(ch) {
        Some(pos) => s_ptr + (pos * 2) as u64,
        None => 0,
    };
    log_red!(emu, "msvcrt!wcschr '{}' '{}' => 0x{:x}", s, ch, result);
    emu.set_last_error(0);
    call.set_return(emu, result);
}
