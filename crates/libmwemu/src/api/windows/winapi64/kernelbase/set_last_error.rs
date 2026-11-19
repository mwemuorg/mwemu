use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!SetLastError err:{}", err);
    helper::advance_tick(emu, 1);
    emu.set_last_error(err);
}
