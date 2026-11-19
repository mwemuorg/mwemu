use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _close(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg(emu, 0));
    let name = helper::handler_get_uri(fd);
    let ok = helper::handler_close(fd);
    log_red!(emu, "msvcrt!_close fd:{} '{}' ok:{}", fd, name, ok);
    call.set_return(emu, if ok { 0 } else { u64::MAX });
}
