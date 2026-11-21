use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn DosPathToSessionPathA(emu: &mut emu::Emu) {
    dos_path_to_session_path(emu, "DosPathToSessionPathA");
}

fn dos_path_to_session_path(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));
    let _lp = api_try!(emu, call.arg_u32(emu, 1));
    let _a3 = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn DosPathToSessionPathW(emu: &mut emu::Emu) {
    dos_path_to_session_path(emu, "DosPathToSessionPathW");
}
