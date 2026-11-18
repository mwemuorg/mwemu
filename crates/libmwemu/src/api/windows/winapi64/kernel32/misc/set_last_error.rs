use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err_code = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!SetLastError err: {}", err_code);
    emu.set_last_error(err_code);
    call.set_return(emu, 1);
}
