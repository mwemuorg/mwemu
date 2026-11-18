use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RaiseException(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let num_args = api_try!(emu, call.arg_u32(emu, 2));
    let args = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!RaiseException {} {}", code, flags);

    call.cleanup_args(emu, 4);

    call.set_return(emu, 0);
    //emu.stop();
}
