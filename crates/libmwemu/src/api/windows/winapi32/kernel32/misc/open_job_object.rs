use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn OpenJobObjectA(emu: &mut emu::Emu) {
    open_job_object(emu, "OpenJobObjectA", false);
}

fn open_job_object(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _dwDesiredAccess = api_try!(emu, call.arg_u32(emu, 0));
    let _bInheritHandle = api_try!(emu, call.arg_u32(emu, 1));
    let lpName = api_try!(emu, call.arg_u32(emu, 2));

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn OpenJobObjectW(emu: &mut emu::Emu) {
    open_job_object(emu, "OpenJobObjectW", true);
}
