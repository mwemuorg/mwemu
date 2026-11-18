use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn getprotobynumber(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let proto = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "ws2_32!getprotobynumber proto:{}", proto);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
