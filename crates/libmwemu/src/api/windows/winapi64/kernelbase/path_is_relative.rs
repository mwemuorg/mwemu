use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathIsRelativeA(emu: &mut emu::Emu) {
    path_is_relative(emu, "PathIsRelativeA", false);
}

fn path_is_relative(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = strings::read_text(emu, path_ptr, wide);
    let is_relative = !path.starts_with("\\") && !path.contains(":\\") && !path.starts_with("/");
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_relative);
    emu.set_last_error(0);
    call.set_return(emu, if is_relative { 1 } else { 0 });
}

pub fn PathIsRelativeW(emu: &mut emu::Emu) {
    path_is_relative(emu, "PathIsRelativeW", true);
}
