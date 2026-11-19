use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _exit(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let status = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!_exit {}", status);
    emu.stop();
}
