use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strchr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s_ptr = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1)) as char;
    let s = emu.maps.read_string(s_ptr);
    let found = s.find(c);
    let result = match found {
        Some(pos) => s_ptr + pos as u64,
        None => 0,
    };
    log_red!(emu, "msvcrt!strchr '{}' '{}' => 0x{:x}", s, c, result);
    emu.set_last_error(0);
    call.set_return(emu, result);
}
