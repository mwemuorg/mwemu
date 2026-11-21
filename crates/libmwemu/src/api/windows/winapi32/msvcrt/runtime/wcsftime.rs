use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn wcsftime(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let wcs = api_try!(emu, call.arg(emu, 0));
    let _maxsize = api_try!(emu, call.arg_u32(emu, 1));
    let fmt_ptr = api_try!(emu, call.arg(emu, 2));
    let _timeptr = api_try!(emu, call.arg_u32(emu, 3));
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    if wcs != 0 && !fmt.is_empty() {
        emu.maps.write_wide_string(wcs, &fmt);
    }
    log_red!(emu, "msvcrt!wcsftime fmt:'{}' => {}", fmt, fmt.len());

    call.set_return(emu, fmt.len() as u64);
}
