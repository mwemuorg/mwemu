use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SendMessageA(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageA");
}

fn send_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let window = api_try!(emu, call.arg_u32(emu, 0));
    let message = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "user32!{api} hwnd:0x{:x} msg:0x{:x}", window, message);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 4);
}

pub(super) fn SendMessageW(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageW");
}
