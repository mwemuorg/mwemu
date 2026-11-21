use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn vwprintf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fmt_ptr = api_try!(emu, call.arg(emu, 0));
    let _va_list = api_try!(emu, call.arg_u32(emu, 1));
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    log_red!(emu, "msvcrt!vwprintf fmt:'{}'", fmt);

    call.set_return(emu, fmt.len() as u64);
}
