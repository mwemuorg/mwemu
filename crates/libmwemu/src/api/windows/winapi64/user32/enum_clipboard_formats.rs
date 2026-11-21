use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EnumClipboardFormats(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let format = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!EnumClipboardFormats format:{}", format);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
