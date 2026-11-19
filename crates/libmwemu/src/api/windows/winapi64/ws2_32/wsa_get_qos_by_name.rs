use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAGetQOSByName(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let qos_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!WSAGetQOSByName sock:0x{:x}", sock);

    emu.set_last_error(0);
    call.set_return(emu, 0);
}
