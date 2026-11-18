use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindClose(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!FindClose");

    call.cleanup_args(emu, 1);

    helper::handler_close(hndl);
    call.set_return(emu, 1);
}
