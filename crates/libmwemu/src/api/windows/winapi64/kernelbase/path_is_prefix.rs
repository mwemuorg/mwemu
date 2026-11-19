use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathIsPrefixA(emu: &mut emu::Emu) {
    path_is_prefix(emu, "PathIsPrefixA", false);
}

fn path_is_prefix(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let prefix_ptr = api_try!(emu, call.arg(emu, 0));
    let path_ptr = api_try!(emu, call.arg(emu, 1));
    let prefix = if prefix_ptr != 0 {
        strings::read_text(emu, prefix_ptr, wide)
    } else {
        String::new()
    };
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_prefix = path.to_lowercase().starts_with(&prefix.to_lowercase());
    log_red!(
        emu,
        "kernelbase!{api} '{}' of '{}' => {}",
        prefix,
        path,
        is_prefix
    );
    emu.set_last_error(0);
    call.set_return(emu, if is_prefix { 1 } else { 0 });
}

pub fn PathIsPrefixW(emu: &mut emu::Emu) {
    path_is_prefix(emu, "PathIsPrefixW", true);
}
