use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetErrorInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _reserved = api_try!(emu, call.arg(emu, 0));
    let perrinfo = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ole32!SetErrorInfo errinfo:0x{:x}", perrinfo);
    for _ in 0..2 {}
    call.set_return(emu, 0); // S_OK
}
