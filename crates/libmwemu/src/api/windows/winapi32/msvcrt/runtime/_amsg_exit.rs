use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _amsg_exit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let code = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
    emu.stop();
}
