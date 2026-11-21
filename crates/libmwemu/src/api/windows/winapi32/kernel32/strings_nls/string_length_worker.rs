use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn StringLengthWorkerA(emu: &mut emu::Emu) {
    string_length_worker(emu, "StringLengthWorkerA");
}

fn string_length_worker(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _psz = api_try!(emu, call.arg_u32(emu, 0));
    let _cchMax = api_try!(emu, call.arg_u32(emu, 1));
    let _pcchLength = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn StringLengthWorkerW(emu: &mut emu::Emu) {
    string_length_worker(emu, "StringLengthWorkerW");
}
