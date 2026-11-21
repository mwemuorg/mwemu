use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetDllDirectoryA(emu: &mut emu::Emu) {
    get_dll_directory(emu, "GetDllDirectoryA");
}

fn get_dll_directory(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _nBufferLength = api_try!(emu, call.arg_u32(emu, 0));
    let _lpBuffer = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn GetDllDirectoryW(emu: &mut emu::Emu) {
    get_dll_directory(emu, "GetDllDirectoryW");
}
