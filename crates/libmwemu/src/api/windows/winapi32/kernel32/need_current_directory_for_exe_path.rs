use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn NeedCurrentDirectoryForExePathA(emu: &mut emu::Emu) {
    need_current_directory_for_exe_path(emu, "NeedCurrentDirectoryForExePathA", false);
}

fn need_current_directory_for_exe_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

pub fn NeedCurrentDirectoryForExePathW(emu: &mut emu::Emu) {
    need_current_directory_for_exe_path(emu, "NeedCurrentDirectoryForExePathW", true);
}
