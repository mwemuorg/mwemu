use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CreateDirectoryA(emu: &mut emu::Emu) {
    create_directory(emu, "CreateDirectoryA", false);
}

fn create_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path = strings::read_text(emu, api_try!(emu, call.arg_u32(emu, 0)) as u64, wide);
    log_red!(emu, "kernel32!{api} `{}` (stub)", path);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}

pub fn CreateDirectoryW(emu: &mut emu::Emu) {
    create_directory(emu, "CreateDirectoryW", true);
}
