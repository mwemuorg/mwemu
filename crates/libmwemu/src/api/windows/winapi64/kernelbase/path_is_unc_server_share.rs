use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathIsUNCServerShareA(emu: &mut emu::Emu) {
    path_is_unc_server_share(emu, "PathIsUNCServerShareA", false);
}

fn path_is_unc_server_share(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_share = if let Some(rest) = path.strip_prefix("\\\\") {
        rest.matches('\\').count() == 1 && !rest.ends_with('\\')
    } else {
        false
    };
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_share);
    emu.set_last_error(0);
    call.set_return(emu, if is_share { 1 } else { 0 });
}

pub fn PathIsUNCServerShareW(emu: &mut emu::Emu) {
    path_is_unc_server_share(emu, "PathIsUNCServerShareW", true);
}
