use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn LZOpenFileA(emu: &mut emu::Emu) {
    lz_open_file(emu, "LZOpenFileA", false);
}

fn lz_open_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpFileName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpReOpenBuf = api_try!(emu, call.arg_u32(emu, 1));
    let _wStyle = api_try!(emu, call.arg_u32(emu, 2));

    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpFileName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn LZOpenFileW(emu: &mut emu::Emu) {
    lz_open_file(emu, "LZOpenFileW", true);
}
