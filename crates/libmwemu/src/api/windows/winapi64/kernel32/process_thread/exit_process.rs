use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ExitProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!ExitProcess code: {}", code);
    emu.stop();
}
