use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let _options = api_try!(emu, call.arg(emu, 0));
    let _stream = api_try!(emu, call.arg(emu, 1));
    let fmt_ptr = api_try!(emu, call.arg(emu, 2));
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!__stdio_common_vfprintf fmt:'{}'", fmt);
    call.set_return(emu, fmt.len() as u64);
}
