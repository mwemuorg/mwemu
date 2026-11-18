use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn StringCopyWorkerA(emu: &mut emu::Emu) {
    string_copy_worker(emu, "StringCopyWorkerA");
}

fn string_copy_worker(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _pszDest = api_try!(emu, call.arg_u32(emu, 0));
    let _cchDest = api_try!(emu, call.arg_u32(emu, 1));
    let _pcchNewDestLength = api_try!(emu, call.arg_u32(emu, 2));
    let _pszSrc = api_try!(emu, call.arg_u32(emu, 3));
    let _cchToCopy = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn StringCopyWorkerW(emu: &mut emu::Emu) {
    string_copy_worker(emu, "StringCopyWorkerW");
}
