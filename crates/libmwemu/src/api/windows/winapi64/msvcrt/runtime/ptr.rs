use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ptr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let p = api_try!(emu, call.arg(emu, 0));
    let val = if p != 0 {
        emu.maps.read_qword(p).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "msvcrt!ptr 0x{:x} => [0x{:x}]", p, val);
    emu.set_last_error(0);
    call.set_return(emu, p);
}
