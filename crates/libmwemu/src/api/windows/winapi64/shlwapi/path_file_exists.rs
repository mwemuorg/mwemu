use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn PathFileExistsA(emu: &mut emu::Emu) {
    path_file_exists(emu, "PathFileExistsA", false);
}

fn path_file_exists(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ptr_path = api_try!(emu, call.arg(emu, 0));
    let path = strings::read_text(emu, ptr_path, wide);

    log_red!(emu, "shlwapi!{api} path: {}", path);

    call.set_return(emu, 1); // True
}

pub(super) fn PathFileExistsW(emu: &mut emu::Emu) {
    path_file_exists(emu, "PathFileExistsW", true);
}
