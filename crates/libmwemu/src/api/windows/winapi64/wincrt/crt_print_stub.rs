use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_print_stub(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let fmt_ptr = api_try!(emu, call.arg(emu, 0));
    let fmt = if api == "wprintf" || api == "vwprintf" {
        emu.maps.read_wide_string(fmt_ptr)
    } else {
        emu.maps.read_string(fmt_ptr)
    };
    log_red!(emu, "wincrt!{} '{}'", api, fmt);
    call.set_return(emu, fmt.len() as u64);
}
