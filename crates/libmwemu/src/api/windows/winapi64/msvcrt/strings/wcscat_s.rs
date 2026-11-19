use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn wcscat_s(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let _dst_size = api_try!(emu, call.arg(emu, 1));
    let src = api_try!(emu, call.arg(emu, 2));
    let existing = emu.maps.read_wide_string(dst);
    let append = emu.maps.read_wide_string(src);
    let result = format!("{}{}", existing, append);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &result);
    }
    log_red!(emu, "msvcrt!wcscat_s => '{}'", result);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
