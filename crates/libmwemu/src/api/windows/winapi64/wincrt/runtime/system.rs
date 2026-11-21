use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn system(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let cmd_ptr = api_try!(emu, call.arg(emu, 0));
    let cmd = if api == "_wsystem" {
        emu.maps.read_wide_string(cmd_ptr)
    } else {
        emu.maps.read_string(cmd_ptr)
    };
    log_red!(emu, "wincrt!{} cmd:'{}'", api, cmd);
    call.set_return(emu, 0);
}
