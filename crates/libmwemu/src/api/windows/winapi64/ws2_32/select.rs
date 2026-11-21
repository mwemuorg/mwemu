use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn select(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let nfds = api_try!(emu, call.arg_u32(emu, 0));
    let readfds = api_try!(emu, call.arg(emu, 1));
    let writefds = api_try!(emu, call.arg(emu, 2));
    let exceptfds = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ws2_32!select nfds:{} read:0x{:x} write:0x{:x}",
        nfds,
        readfds,
        writefds
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
