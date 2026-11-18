use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _sprintf_l(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buf = api_try!(emu, call.arg(emu, 0));
    let fmt_ptr = api_try!(emu, call.arg(emu, 1));
    let _locale = api_try!(emu, call.arg_u32(emu, 2));
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!_sprintf_l buf:0x{:x} fmt:'{}'", buf, fmt);
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }

    call.set_return(emu, fmt.len() as u64);
}
