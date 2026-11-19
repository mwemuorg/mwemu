use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DispCallFunc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "oleaut32!DispCallFunc 0x{:x}", punk);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
