use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileA");
}

fn find_next_file(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg_u32(emu, 0));
    let find_data = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!{api}");

    call.cleanup_args(emu, 2);

    call.set_return(emu, ERROR_NO_MORE_FILES);
}

pub fn FindNextFileW(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileW");
}
