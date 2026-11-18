use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateToolhelp32Snapshot(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg(emu, 0));
    let pid = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "kernel32!CreateToolhelp32Snapshot flags: {:x} pid: {}",
        flags,
        pid
    );

    let uri = format!("CreateToolhelp32Snapshot://{}", pid);
    call.set_return(emu, helper::handler_create(&uri));
}
