use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    add_font_resource_ex(emu, "AddFontResourceExA", false);
}

fn add_font_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "gdi32!{api} '{}'", name);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}

pub fn AddFontResourceExW(emu: &mut emu::Emu) {
    add_font_resource_ex(emu, "AddFontResourceExW", true);
}
