use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetFullPathNameTransactedA(emu: &mut emu::Emu) {
    get_full_path_name_transacted(emu, "GetFullPathNameTransactedA", false);
}

fn get_full_path_name_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpFileName = api_try!(emu, call.arg_u32(emu, 0));
    let _nBufferLength = api_try!(emu, call.arg_u32(emu, 1));
    let _lpBuffer = api_try!(emu, call.arg_u32(emu, 2));
    let _lpFilePart = api_try!(emu, call.arg_u32(emu, 3));
    let _hTransaction = api_try!(emu, call.arg_u32(emu, 4));

    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpFileName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn GetFullPathNameTransactedW(emu: &mut emu::Emu) {
    get_full_path_name_transacted(emu, "GetFullPathNameTransactedW", true);
}
