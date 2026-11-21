use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DragAcceptFiles(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let accept = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "shell32!DragAcceptFiles hwnd:0x{:x} accept:{}",
        hwnd,
        accept
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
