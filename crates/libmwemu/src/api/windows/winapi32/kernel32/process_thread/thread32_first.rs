use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn Thread32First(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg_u32(emu, 0));
    let entry = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!Thread32First");

    call.cleanup_args(emu, 2);

    call.set_return(emu, 1);
}
