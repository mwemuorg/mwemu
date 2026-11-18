use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathIsUNCServerA(emu: &mut emu::Emu) {
    path_is_unc_server(emu, "PathIsUNCServerA", false);
}

fn path_is_unc_server(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_server = path.starts_with("\\\\") && !path[2..].contains('\\');
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_server);
    emu.set_last_error(0);
    call.set_return(emu, if is_server { 1 } else { 0 });
}

pub fn PathIsUNCServerW(emu: &mut emu::Emu) {
    path_is_unc_server(emu, "PathIsUNCServerW", true);
}
