use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn PostQuitMessage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let exit_code = api_try!(emu, call.arg_i32(emu, 0));
    log_red!(emu, "user32!PostQuitMessage code:{}", exit_code);
}
