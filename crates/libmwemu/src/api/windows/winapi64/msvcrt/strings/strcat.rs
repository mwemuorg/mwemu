use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strcat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let existing = emu.maps.read_string(dst);
    let append = emu.maps.read_string(src);
    let result = format!("{}{}", existing, append);
    if dst != 0 {
        emu.maps.write_string(dst, &result);
    }
    log_red!(emu, "msvcrt!strcat => '{}'", result);
    emu.set_last_error(0);
    call.set_return(emu, dst);
}
