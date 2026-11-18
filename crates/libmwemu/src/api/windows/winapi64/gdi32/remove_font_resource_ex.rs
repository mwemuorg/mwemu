use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExA", false);
}

fn remove_font_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name = strings::read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let fl = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "gdi32!{api} '{}' flags:0x{:x}", name, fl);
    call.set_return(emu, 1);
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExW", true);
}
