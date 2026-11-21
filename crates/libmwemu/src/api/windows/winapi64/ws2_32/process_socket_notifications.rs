use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ProcessSocketNotifications(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let completion_port = api_try!(emu, call.arg(emu, 0));
    let count = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "ws2_32!ProcessSocketNotifications port:0x{:x} count:{}",
        completion_port,
        count
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
