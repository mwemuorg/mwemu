use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn InitOnceInitialize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!InitOnceInitialize");
    let handle = helper::handler_create("InitOnceInitialize");
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
