use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg(emu, 0));
    let inherit = api_try!(emu, call.arg(emu, 1));
    let pid = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "kernel32!OpenProcess pid: {}", pid);

    let uri = format!("pid://{}", pid);
    call.set_return(emu, helper::handler_create(&uri));
}
