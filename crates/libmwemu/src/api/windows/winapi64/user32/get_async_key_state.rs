use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetAsyncKeyState(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let vkey = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "user32!GetAsyncKeyState vKey=0x{:x}", vkey);
    call.set_return(emu, 0); // key not pressed
}
