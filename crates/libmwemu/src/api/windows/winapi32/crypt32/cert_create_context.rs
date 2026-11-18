use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertCreateContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    let arg5 = api_try!(emu, call.arg(emu, 4));
    call.cleanup_args(emu, 6);
    log_red!(
        emu,
        "crypt32!CertCreateContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5
    );
    call.set_return(emu, helper::handler_create("CertCreateContext"));
}
