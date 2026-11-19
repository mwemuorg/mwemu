use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg_u32(emu, 0));
    let inherit = api_try!(emu, call.arg_u32(emu, 1));
    let pid = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!OpenProcess pid: {}", pid);

    call.cleanup_args(emu, 3);

    let uri = format!("pid://{}", pid);
    call.set_return(emu, helper::handler_create(&uri));
}
