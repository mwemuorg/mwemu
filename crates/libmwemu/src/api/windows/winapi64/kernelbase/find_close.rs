use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindClose(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hfind = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!FindClose handle:0x{:x}", hfind);
    if helper::handler_exist(hfind) {
        helper::handler_close(hfind);
    }
    call.set_return(emu, 1);
}
