use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WEP(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reason = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WEP reason:{}", reason);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
