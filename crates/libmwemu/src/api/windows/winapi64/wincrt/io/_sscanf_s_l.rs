use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _sscanf_s_l(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buf_ptr = api_try!(emu, call.arg(emu, 0));
    let fmt_ptr = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(buf_ptr);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "wincrt!_sscanf_s_l buf:'{}' fmt:'{}'", s, fmt);
    call.set_return(emu, 1);
}
