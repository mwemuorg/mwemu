use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetDllDirectoryA(emu: &mut emu::Emu) {
    set_dll_directory(emu, "SetDllDirectoryA", false);
}

fn set_dll_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpPathName = api_try!(emu, call.arg_u32(emu, 0));

    let lpPathName = strings::read_text(emu, lpPathName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpPathName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 1);
}

pub fn SetDllDirectoryW(emu: &mut emu::Emu) {
    set_dll_directory(emu, "SetDllDirectoryW", true);
}
