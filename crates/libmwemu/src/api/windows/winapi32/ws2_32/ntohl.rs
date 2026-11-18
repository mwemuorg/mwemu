use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn ntohl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let val = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ws2_32!ntohl 0x{:x}", val);
    call.set_return(emu, u32::from_be(val) as u64);
    call.cleanup_args(emu, 1);
}
