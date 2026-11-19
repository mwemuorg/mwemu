use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_itoa(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let val = api_try!(emu, call.arg_i32(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let radix = api_try!(emu, call.arg_u32(emu, 2));
    let s = match radix {
        16 => format!("{:x}", val),
        8 => format!("{:o}", val),
        _ => format!("{}", val),
    };
    if buf != 0 {
        emu.maps.write_string(buf, &s);
    }
    log_red!(emu, "wincrt!_itoa {} radix:{} => '{}'", val, radix, s);
    call.set_return(emu, buf);
}
