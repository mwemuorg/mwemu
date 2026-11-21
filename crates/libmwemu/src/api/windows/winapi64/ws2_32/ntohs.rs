use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ntohs(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let val = api_try!(emu, call.arg_u16(emu, 0));
    log_red!(emu, "ws2_32!ntohs val:0x{:x}", val);
    emu.set_last_error(0);
    call.set_return(emu, u16::from_be(val) as u64);
}
