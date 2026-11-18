use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn accept(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let saddr = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(emu, "ws2_32!accept  connections: {}", bytes);

    call.cleanup_args(emu, 3);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }
}
