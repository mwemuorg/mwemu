use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn listen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let connections = api_try!(emu, call.arg(emu, 1));

    log_red!(emu, "ws2_32!listen  connections: {}", connections);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }
}
