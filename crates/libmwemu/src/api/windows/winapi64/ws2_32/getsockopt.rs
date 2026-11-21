use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn getsockopt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let level = api_try!(emu, call.arg(emu, 1));
    let optname = api_try!(emu, call.arg(emu, 2));
    let optval = api_try!(emu, call.arg(emu, 3));
    let optlen = api_try!(emu, call.arg(emu, 4));

    emu.maps.write_dword(optval, 1);

    log_red!(emu, "ws2_32!getsockopt  lvl: {} opt: {}", level, optname);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }
}
