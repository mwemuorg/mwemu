use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SendMessageTimeoutA(emu: &mut emu::Emu) {
    send_message_timeout(emu, "SendMessageTimeoutA", false);
}

fn send_message_timeout(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let name = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "user32!{api} '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SendMessageTimeoutW(emu: &mut emu::Emu) {
    send_message_timeout(emu, "SendMessageTimeoutW", true);
}
