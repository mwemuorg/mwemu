use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn fputs(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let stream = api_try!(emu, call.arg(emu, 1));
    let s = if str_ptr != 0 {
        emu.maps.read_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wincrt!fputs '{}' stream:0x{:x}", s, stream);
    emu.set_last_error(0);
    call.set_return(emu, s.len() as u64);
}
