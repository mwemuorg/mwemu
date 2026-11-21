use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathIsFileSpecA(emu: &mut emu::Emu) {
    path_is_file_spec(emu, "PathIsFileSpecA", false);
}

fn path_is_file_spec(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let is_filespec = !path.contains('\\') && !path.contains('/') && !path.contains(':');
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, is_filespec);
    emu.set_last_error(0);
    call.set_return(emu, if is_filespec { 1 } else { 0 });
}

pub fn PathIsFileSpecW(emu: &mut emu::Emu) {
    path_is_file_spec(emu, "PathIsFileSpecW", true);
}
