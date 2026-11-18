use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenClipboard(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd_owner = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("clipboard");
    log_red!(
        emu,
        "user32!OpenClipboard owner:0x{:x} => 0x{:x}",
        hwnd_owner,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
