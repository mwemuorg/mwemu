use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn setsockopt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let level = api_try!(emu, call.arg(emu, 1));
    let optname = api_try!(emu, call.arg(emu, 2));
    let optval = api_try!(emu, call.arg(emu, 3));
    let optlen = api_try!(emu, call.arg(emu, 4));

    let val = emu.maps.read_dword(optval).unwrap_or_default();

    log_red!(
        emu,
        "ws2_32!setsockopt  lvl: {} opt: {} val: {}",
        level,
        optname,
        val
    );

    call.cleanup_args(emu, 5);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }
}
