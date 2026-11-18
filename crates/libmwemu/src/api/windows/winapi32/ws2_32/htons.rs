use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn htons(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let port: u16 = api_try!(emu, call.arg_u16(emu, 0));

    log_red!(emu, "ws2_32!htons port: {}", port);

    call.cleanup_args(emu, 1);
    call.set_return(emu, port.to_be() as u64);
}
