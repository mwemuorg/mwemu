use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathIsURLA(emu: &mut emu::Emu) {
    path_is_url(emu, "PathIsURLA", false);
}

fn path_is_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_url = path.contains("://");
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_url);
    emu.set_last_error(0);
    call.set_return(emu, if is_url { 1 } else { 0 });
}

pub fn PathIsURLW(emu: &mut emu::Emu) {
    path_is_url(emu, "PathIsURLW", true);
}
