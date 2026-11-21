use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn swprintf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buf = api_try!(emu, call.arg(emu, 0));
    let fmt_ptr = api_try!(emu, call.arg(emu, 1));
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    if buf != 0 {
        emu.maps.write_wide_string(buf, &fmt);
    }
    log_red!(emu, "wincrt!swprintf fmt:'{}'", fmt);
    call.set_return(emu, fmt.len() as u64);
}
