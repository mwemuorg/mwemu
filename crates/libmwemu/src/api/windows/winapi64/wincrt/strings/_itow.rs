use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _itow(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let value = api_try!(emu, call.arg_i32(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let radix = api_try!(emu, call.arg_u32(emu, 2));
    let s = if radix == 16 {
        format!("{:x}", value)
    } else {
        format!("{}", value)
    };
    if buf != 0 {
        emu.maps.write_wide_string(buf, &s);
    }
    log_red!(emu, "wincrt!_itow {} radix:{} => '{}'", value, radix, s);
    call.set_return(emu, buf);
}
