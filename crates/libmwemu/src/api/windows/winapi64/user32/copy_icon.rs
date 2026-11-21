use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CopyIcon(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hicon = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!CopyIcon hIcon:0x{:x}", hicon);
    let hndl = helper::handler_create("user32://icon");
    call.set_return(emu, hndl);
}
