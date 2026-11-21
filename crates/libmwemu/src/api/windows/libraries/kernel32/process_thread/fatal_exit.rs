use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;

pub fn FatalExit(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg_i32(emu, 0));
    log_red!(emu, "FatalExit code={}", code);
    call.cleanup_args(emu, 1);
    emu.stop();
}
